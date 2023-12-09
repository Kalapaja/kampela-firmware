//! Everything high-level related to interfacing with user

use nalgebra::{Affine2, OMatrix, Point2, RowVector3};
use alloc::vec::Vec;
use alloc::string::String;
use lazy_static::lazy_static;

use kampela_system::{
    in_free,
    if_in_free,
    devices::{se_rng, se_aes_gcm, touch::{Read, LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES}},
    draw::FrameBuffer,
    parallel::Operation,
};
use kampela_system::devices::flash::*;

use kampela_ui::{display_def::*, uistate, pin::Pincode, platform::{NfcTransaction, Platform}};
use embedded_graphics::prelude::Point;

/// UI handler
pub struct UI {
    pub state: uistate::UIState<Hardware>,
    status: UIStatus,
    update: uistate::UpdateRequest,
}

impl UI {
    /// Start of UI.
    pub fn init() -> Self {
        let mut update = uistate::UpdateRequest::new();
        update.set_slow();
        let hardware = Hardware::new();
        let state = uistate::UIState::new(hardware);
        return Self {
            state: state,
            status: UIStatus::Listen,
            update: update,
        }
    }

    /// Call in event loop to progress through UI state
    pub fn advance(&mut self, voltage: i32) {
        match self.status {
            UIStatus::Listen => {
                self.listen();
            },
            UIStatus::DisplayOperation => {
                if self.state.display().advance(voltage) {
                    self.status = UIStatus::Listen;
                }
            },
            UIStatus::TouchOperation(ref mut touch) => {
                match touch.advance(()) {
                    Ok(Some(touch)) => if let Some(point) = convert(touch) {
                        self.update = self.state.handle_tap::<FrameBuffer>(point, &mut ()).unwrap();
                        self.status = UIStatus::Listen;
                    },
                    Ok(None) => {},
                    Err(e) => panic!("{:?}", e),
                }
            },
        }
    }

    fn listen(&mut self) {
        // 1. update ui if needed
        let f = self.update.read_fast();
        let s = self.update.read_slow();

        if f || s {
            self.update = self.state.render::<FrameBuffer>().expect("guaranteed to work, no errors implemented");
            self.status = UIStatus::DisplayOperation;
        }
        if f {
            self.state.display().request_fast();
            return;
        }
        if s {
            self.state.display().request_full();
            return;
        }
        // 2. read input if possible
        if if_in_free(|peripherals|
            peripherals.GPIO_S.if_.read().extif0().bit_is_set()
        ).unwrap() {
            self.status = UIStatus::TouchOperation(Read::new());
        };
    }

    pub fn handle_transaction(&mut self, transaction: NfcTransaction) {
        self.update = self.state.handle_transaction(&mut se_rng::SeRng{}, transaction);
    }

    pub fn handle_address(&mut self, addr: [u8; 76]) {
        self.update = self.state.handle_address(addr);
    }
}

/// General status of UI
///
/// There is no sense in reading input while screen processes last event, nor refreshing the screen
/// before touch was parsed
enum UIStatus {
    /// Event listening state, default
    Listen,
    /// Screen update started
    DisplayOperation,
    /// Touch event processing
    TouchOperation(Read<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>),
}

pub struct Hardware {
    pin: Pincode,
    pub entropy: Vec<u8>,
    display: FrameBuffer,
    call: Option<String>,
    extensions: Option<String>,
    signature: Option<[u8; 130]>,
    address: Option<[u8; 76]>,
}

impl Hardware {
    pub fn new() -> Self {
        let entropy = Vec::new();
        let pin_set = false; // TODO query storage
        let pin = Pincode::new(&mut Self::rng(&mut ()), pin_set);
        let display = FrameBuffer::new_white();
        Self {
            pin: pin,
            entropy: entropy,
            display: display,
            call: None,
            extensions: None,
            signature: None,
            address: None,
        }
    }
}

impl <'a> Platform for Hardware {
    type HAL = ();
    type Rng<'c> = se_rng::SeRng;
    type Display = FrameBuffer;

    fn rng<'b>(_: &'b mut ()) -> Self::Rng<'static> {
        se_rng::SeRng{}
    }

    fn pin(&self) -> &Pincode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut Pincode {
        &mut self.pin
    }

    fn display(&mut self) -> &mut <Self as Platform>::Display {
        &mut self.display
    }

    fn store_entropy(&mut self) {
        let len = self.entropy.len();
        //let mut entropy_storage = [0u8; 33];
        //entropy_storage[0] = len.try_into().expect("entropy is at most 32 bytes");
        //entropy_storage[1..1+len].copy_from_slice(&self.entropy);
        // TODO encode
        in_free(|peripherals| {
            se_aes_gcm::create_key(peripherals).unwrap();
            let protected = se_aes_gcm::aes_gcm_encrypt(
                peripherals,
                [0; se_aes_gcm::AAD_LEN],
                [0; se_aes_gcm::IV_LEN],
                self.entropy.clone(),
            ).unwrap();

            let mut storage_payload = [0u8; 1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN];
            storage_payload[0] = protected.len as u8;
            storage_payload[1..1+se_aes_gcm::SECRET_MAX_LEN].copy_from_slice(&protected.data);
            storage_payload[1+se_aes_gcm::SECRET_MAX_LEN..1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN].copy_from_slice(&protected.tag);
            storage_payload[1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN..].copy_from_slice( unsafe { &se_aes_gcm::KEY_BUFFER });

            flash_wakeup(peripherals);

            flash_unlock(peripherals);
            flash_erase_page(peripherals, 0);
            flash_wait_ready(peripherals);

            flash_unlock(peripherals);
            flash_write_page(peripherals, 0, &storage_payload);
            flash_wait_ready(peripherals);

            let mut ent = [0u8; 2+1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN];
            flash_read(peripherals, 0, &mut ent);

            // Incorrect behavior of flash after wakeup. It reads two bytes of zeroes before the actual data stored in flash.
            if ent[2..] != storage_payload {
                panic!("Failed to save seedphrase: {:?} ||| {:?}", &ent[2..35], &self.entropy[..33]);
            }
        });
    }

    fn read_entropy(&mut self) {
        let mut ent = [0u8; 1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN];
        in_free(|peripherals| {
            // Make sure that flash is ok
            flash_wakeup(peripherals);
            flash_wait_ready(peripherals);
            let fl_id = flash_get_id(peripherals);
            let fl_len = flash_get_size(peripherals);
            if (fl_id == 0) || (fl_len == 0) {
                panic!("Flash error");
            }
            flash_read(peripherals, 0, &mut ent);
            flash_sleep(peripherals);
        });
        match ent[0] {
            0 => self.entropy = Vec::new(),
            16 | 20 | 24 | 28 | 32 => {
                let recovered_out = se_aes_gcm::Out {
                    data: ent[1..1+se_aes_gcm::SECRET_MAX_LEN].try_into().expect("static length"),
                    len: ent[0] as usize,
                    tag: ent[1+se_aes_gcm::SECRET_MAX_LEN..1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN].try_into().expect("static length"),
                };
                unsafe { se_aes_gcm::KEY_BUFFER = ent[1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN..].try_into().expect("static length"); }
                in_free(|peripherals| {
                    let out = se_aes_gcm::aes_gcm_decrypt(
                        peripherals,
                        &recovered_out,
                        [0u8; se_aes_gcm::AAD_LEN],
                        [0u8; se_aes_gcm::IV_LEN],
                    ).unwrap();
                    self.entropy = out.data[..out.len].to_vec();
                })
            },
            255 => self.entropy = Vec::new(),
            _ => {
                self.store_entropy();
                panic!("Seed storage corrupted! Wiping seed...");
            },
        }
    }

    fn pin_display(&mut self) -> (&mut Pincode, &mut <Self as Platform>::Display) {
        (&mut self.pin, &mut self.display)
    }

    fn set_entropy(&mut self, e: &[u8]) {
        self.entropy = e.to_vec(); // TODO: dedicated array storage maybe
    }

    fn entropy(&self) -> &[u8] {
        &self.entropy
    }

    fn entropy_display(&mut self) -> (&[u8], &mut <Self as Platform>::Display) {
        (&self.entropy, &mut self.display)
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_transaction(&mut self, call: String, extensions: String, signature: [u8; 130]) {
        self.call = Some(call);
        self.extensions = Some(extensions);
        self.signature = Some(signature);
    }


    fn call(&mut self) -> Option<(&str, &mut <Self as Platform>::Display)> {
        match &self.call {
            Some(a) => Some((a.as_str(), &mut self.display)),
            None => None,
        }
    }

    fn extensions(&mut self) -> Option<(&str, &mut <Self as Platform>::Display)> {
        match &self.extensions {
            Some(a) => Some((a.as_str(), &mut self.display)),
            None => None,
        }
    }

    fn signature(&mut self) -> (&[u8; 130], &mut <Self as Platform>::Display) {
        if let Some(ref a) = self.signature {
            (a, &mut self.display)
        } else {
            panic!("qr generation failed");
        }
    }

    fn address(&mut self) -> (&[u8; 76], &mut <Self as Platform>::Display) {
        if let Some(ref a) = self.address {
            (a, &mut self.display)
        } else {
            panic!("qr generation failed");
        }
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

        Some(
            Point {
                x: display_as_point2.coords[0] as i32,
                y: display_as_point2.coords[1] as i32,
            }
        )
    } else { None }
}


