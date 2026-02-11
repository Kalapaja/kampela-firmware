//! Everything high-level related to interfacing with user

use alloc::{collections::VecDeque, string::String, vec::Vec};
use embedded_graphics::{geometry::Dimensions, prelude::Point};
use lazy_static::lazy_static;
use nalgebra::{Affine2, OMatrix, Point2, RowVector3};

use alloy_primitives::Address;
use kampela_system::devices::flash::*;
use kampela_system::{
    devices::{
        se_aes_gcm::{decode_entropy, encode_entropy, Protected},
        se_rng,
        touch::{touch_detected, Read, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES},
    },
    draw::FrameBuffer,
    parallel::Operation,
};
use kampela_ui::{
    display_def::*,
    error::KampelaError,
    eth_transaction::{
        derive_eth_address, format_eth_transaction_display, sign_eip1559_transaction,
        EthTransaction,
    },
    platform::{PinCode, Platform},
    uistate::{UIState, UpdateRequest, UpdateRequestMutate},
};

const MAX_TOUCH_QUEUE: usize = 2;

/// UI handler
pub struct UI {
    pub state: UIState<Hardware, FrameBuffer>,
    status: UIStatus,
    touches: VecDeque<Point>,
    touched: bool,
    update_request: Option<UpdateRequest>,
}

impl UI {
    /// Start of UI.
    pub fn init() -> Self {
        let hardware = Hardware::new();
        let display = FrameBuffer::new_white();
        let state = UIState::new(hardware, display, &mut ());
        Self {
            state,
            status: UIStatus::DisplayOrListen(UIStatusDisplay::Listen),
            touches: VecDeque::new(),
            touched: false,
            update_request: Some(UpdateRequest::Slow),
        }
    }

    /// Call in event loop to progress through UI state
    pub fn advance(&mut self, voltage: i32) -> Option<bool> {
        match self.status {
            UIStatus::DisplayOrListen(ref mut status) => {
                // read input if possible
                if touch_detected().unwrap_or(false) {
                    if !self.touched
                        && !matches!(
                            status,
                            UIStatusDisplay::DisplayOperation(UpdateRequest::Slow)
                        )
                    {
                        self.touched = true;
                        self.status =
                            UIStatus::TouchOperation(Read::new(()), core::mem::take(status));
                        return None;
                    }
                } else {
                    self.touched = false;
                }
                match status {
                    UIStatusDisplay::Listen => {
                        self.listen();
                        Some(true) // done operations
                    }
                    UIStatusDisplay::DisplayOperation(_) => {
                        match self.state.display.advance(voltage) {
                            Some(c) => {
                                if c {
                                    self.status =
                                        UIStatus::DisplayOrListen(UIStatusDisplay::Listen);
                                }
                                Some(false)
                            }
                            None => None, // not enough energy to start screen update
                        }
                    }
                }
            }
            UIStatus::TouchOperation(ref mut touch, ref mut next) => match touch.advance(()) {
                Ok(Some(touch)) => {
                    if self.touches.len() < MAX_TOUCH_QUEUE {
                        if let Some(point) = convert(touch) {
                            self.touches.push_back(point);
                        }
                    }
                    self.status = UIStatus::DisplayOrListen(core::mem::take(next));
                    None
                }
                Ok(None) => None,
                Err(e) => panic!("{:?}", e),
            },
        }
    }

    fn listen(&mut self) {
        if let Some(point) = self.touches.pop_front() {
            self.update_request
                .propagate(self.state.handle_tap(point, &mut ()));
        }
        // update ui if needed
        if let Some(u) = self.update_request.take() {
            let is_clear_update =
                matches!(u, UpdateRequest::Slow) || matches!(u, UpdateRequest::Fast);
            // Handle render errors gracefully - errors are already displayed via ErrorDialog
            if let Ok(result) = self.state.render(is_clear_update, &mut ()) {
                self.update_request.propagate(result);
            }

            match u {
                UpdateRequest::Hidden => (),
                UpdateRequest::Slow => self.state.display.request_full(),
                UpdateRequest::Fast => self.state.display.request_fast(),
                UpdateRequest::UltraFast => {
                    let a = self.state.display.bounding_box();
                    self.state.display.request_part(a);
                }
                UpdateRequest::Part(a) => self.state.display.request_part(a),
            }
            if !matches!(u, UpdateRequest::Hidden) {
                self.status = UIStatus::DisplayOrListen(UIStatusDisplay::DisplayOperation(u));
            }
        }
    }

    pub fn handle_error_message(&mut self, message: String) {
        let error = KampelaError::Other(message);
        self.update_request
            .propagate(self.state.handle_error(error, &mut ()));
    }

    pub fn handle_message(&mut self, title: &str, message: &str) {
        self.update_request
            .propagate(self.state.handle_message(title, message, &mut ()));
    }

    pub fn handle_eth_transaction(&mut self, transaction: EthTransaction) {
        self.state.platform.eth_set_transaction(transaction);
        self.update_request
            .propagate(self.state.handle_transaction(&mut ()));
    }

    pub fn handle_test_message(&mut self, message: String) {
        // Display the string message directly on the test message screen
        self.update_request.propagate(
            self.state.handle_test_message(message, &mut ())
        );
    }
}

/// General status of UI
///
/// There is no sense in reading input while screen processes last event, nor refreshing the screen
/// before touch was parsed

#[derive(Default)]
enum UIStatusDisplay {
    /// Event listening state, default
    #[default]
    Listen,
    /// Screen update started
    DisplayOperation(UpdateRequest),
}
enum UIStatus {
    DisplayOrListen(UIStatusDisplay),
    /// Touch event processing
    TouchOperation(
        Read<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>,
        UIStatusDisplay,
    ),
}
pub struct Hardware {
    pin: PinCode,
    protected: Option<Protected>,
    eth_address: Option<Address>,
    eth_transaction: Option<EthTransaction>,
    eth_transaction_signed: Option<Vec<u8>>,
}

impl Hardware {
    pub fn new() -> Self {
        let _pin_set = false; // TODO query storage
        let pin = [0; 4];
        Self {
            pin,
            protected: None,
            eth_address: None,
            eth_transaction: None,
            eth_transaction_signed: None,
        }
    }
}

impl Platform for Hardware {
    type HAL = ();
    type Rng<'c> = se_rng::SeRng;
    type EthTransaction = EthTransaction;

    fn rng(_: &mut ()) -> Self::Rng<'static> {
        se_rng::SeRng {}
    }

    fn pin(&self) -> &PinCode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut PinCode {
        &mut self.pin
    }

    fn store_entropy(&mut self, e: &[u8]) -> Result<(), KampelaError> {
        let protected = encode_entropy(e);
        store_encoded_entopy(&protected);
        self.protected = Some(protected);

        Ok(())
    }

    fn read_entropy(&mut self) -> Result<(), KampelaError> {
        if let Some(protected) = read_encoded_entropy() {
            self.protected = Some(protected);
            Ok(())
        } else {
            Err(KampelaError::FlashRead)
        }
    }

    fn entropy(&self) -> Result<Vec<u8>, KampelaError> {
        let protected = self.protected.as_ref().ok_or(KampelaError::FlashRead)?;

        Ok(decode_entropy(protected))
    }

    fn eth_address(&self) -> Result<Address, KampelaError> {
        if let Some(addr) = self.eth_address {
            return Ok(addr);
        }
        let entropy = self.entropy()?;
        let addr = derive_eth_address(&entropy)?;
        Ok(addr)
    }

    fn eth_set_address(&mut self, addr: Address) {
        self.eth_address = Some(addr);
    }

    fn eth_set_transaction(&mut self, transaction: Self::EthTransaction) {
        self.eth_transaction = Some(transaction);
    }

    fn eth_transaction(&self) -> Result<&Self::EthTransaction, KampelaError> {
        self.eth_transaction.as_ref().ok_or_else(|| {
            use kampela_ui::error::kampela_error;
            kampela_error!(TransactionInvalid, "No transaction")
        })
    }

    fn eth_transaction_display(&self) -> Result<String, KampelaError> {
        let tx = self.eth_transaction.as_ref().ok_or_else(|| {
            use kampela_ui::error::kampela_error;
            kampela_error!(TransactionInvalid, "No transaction")
        })?;
        let sender = self.eth_address()?;

        format_eth_transaction_display(tx, sender).map_err(|e| {
            use kampela_ui::error::kampela_error;
            kampela_error!(TransactionInvalid, "{}", e)
        })
    }

    fn eth_sign_transaction(&mut self) -> Result<Vec<u8>, KampelaError> {
        let tx = self.eth_transaction.as_ref().ok_or_else(|| {
            use kampela_ui::error::kampela_error;
            kampela_error!(TransactionInvalid, "No transaction")
        })?;
        let entropy = self.entropy()?;

        let signed_tx = sign_eip1559_transaction(tx, &entropy)?;
        self.eth_transaction_signed = Some(signed_tx.clone());
        Ok(signed_tx)
    }
}

lazy_static! {
    // MAGIC calibration numbers obtained through KOLIBRI tool
    static ref AFFINE_MATRIX: Affine2<f32> = Affine2::from_matrix_unchecked(
        OMatrix::from_rows(&[
            RowVector3::<f32>::new(1.0022, -0.0216, -4.2725),
            RowVector3::<f32>::new(0.0061, 1.1433, -13.7305),
            RowVector3::<f32>::new(0.0, 0.0, 1.0),
        ])
    );
}

pub fn convert(touch_data: [u8; LEN_NUM_TOUCHES]) -> Option<Point> {
    if touch_data[0] == 1 {
        let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
        let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
        let touch = Point::new(SCREEN_SIZE_X as i32 - detected_x, detected_y);

        let touch_as_point2 = Point2::new(touch.x as f32, touch.y as f32);
        let display_as_point2 = AFFINE_MATRIX.transform_point(&touch_as_point2);

        Some(Point {
            x: display_as_point2.coords[0] as i32,
            y: display_as_point2.coords[1] as i32,
        })
    } else {
        None
    }
}
