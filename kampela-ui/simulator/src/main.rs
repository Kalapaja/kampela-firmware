//! This is simulator to develop Kampela UI mocks
#![deny(unused_crate_dependencies)]
use embedded_graphics_core::{
    primitives::PointsIter,
    Drawable,
    pixelcolor::BinaryColor,
    Pixel,
};

use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::{rngs::ThreadRng, thread_rng};
use std::{collections::VecDeque, thread::sleep, time::Duration, str::FromStr};
use clap::Parser;
use alloy_primitives::Address;
mod sample_eth_tx;
use sample_eth_tx::sample_eth_transaction;

/// Amount of time required for full screen update; debounce
///  should be quite large as screen takes this much to clean
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);
const BLINK_UPDATE_TIME: Duration = Duration::new(0, 5000000);
const SLOW_UPDATE_ITER: usize = 8;
const FAST_UPDATE_TIME: Duration = Duration::new(1, 0);
const ULTRAFAST_UPDATE_TIME: Duration = Duration::new(1, 0);
const UPDATE_DELAY_TIME: Duration = Duration::new(0, 500000000);

const MAX_TOUCH_QUEUE: usize = 2;

use kampela_ui::{
    data_state::{AppStateInit, NFCState, DataInit, StorageState},
    display_def::*,
    error::KampelaError,
    eth_transaction::{
        derive_eth_address, format_eth_transaction_display, sign_eip1559_transaction, EthTransaction,
    },
    platform::{PinCode, Platform},
    uistate::{UIState, UpdateRequest, UpdateRequestMutate},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'I')]
    key_was_created: bool,

    #[arg(short = 'E')]
    eth_transaction_received: bool,

    #[arg(short = 'T')]
    test_message_received: bool,
}

impl DataInit<Args> for AppStateInit {
    fn new(params: Args) -> AppStateInit {
        let storage = StorageState {
            key_created: params.key_was_created,
        };

        let nfc = if params.test_message_received {
            NFCState::TestMessage
        } else if params.eth_transaction_received {
            NFCState::EthTransaction
        } else {
            NFCState::Empty
        };

        AppStateInit {
            nfc: nfc,
            storage: storage,
        }
    }
}

struct HALHandle {
    pub rng: ThreadRng,
}

impl HALHandle {
    pub fn new() -> Self {
        let rng = thread_rng();
        Self {
            rng: rng,
        }
    }
}

#[derive(Debug)]
struct DesktopSimulator {
    pin: PinCode,
    entropy: Option<Vec<u8>>,
    eth_address: Option<Address>,
    eth_transaction: Option<EthTransaction>,
    signed_tx: Option<Vec<u8>>,
    test_message: Option<String>,
}

impl DesktopSimulator {
    pub fn new(init_state: &AppStateInit) -> Self {
        let pin = [0; 4];

        let (eth_transaction, eth_address, entropy) = if matches!(init_state.nfc, NFCState::EthTransaction) {
            // For transaction testing, generate random entropy (same as device)
            let mut hal_handle = HALHandle::new();
            let random_entropy = Self::generate_entropy(&mut hal_handle);
            let entropy_vec = random_entropy.to_vec();

            // Derive address from this entropy
            let addr = derive_eth_address(&entropy_vec).ok();

            (Some(sample_eth_transaction()), addr, Some(entropy_vec))
        } else {
            (None, None, None)
        };

        let test_message = if matches!(init_state.nfc, NFCState::TestMessage) {
            Some("Hello from NFC simulator!\n\nThis is a test message to demonstrate the dynamic text display feature.\n\nYou can send messages of any length!".to_string())
        } else {
            None
        };

        Self {
            pin,
            entropy,
            eth_address,
            eth_transaction,
            signed_tx: None,
            test_message,
        }
    }
}

impl Platform for DesktopSimulator {
    type HAL = HALHandle;
    type Rng<'a> = &'a mut ThreadRng;
    type EthTransaction = EthTransaction;

    fn rng<'a>(h: &'a mut Self::HAL) -> Self::Rng<'a> {
        &mut h.rng
    }

    fn pin(&self) -> &PinCode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut PinCode {
        &mut self.pin
    }

    fn store_entropy(&mut self, e: &[u8]) -> Result<(), KampelaError> {
        println!("Entropy stored (simulator - not persisted)");
        self.entropy = Some(e.to_vec());

        // Derive and cache Ethereum address
        let address = derive_eth_address(e)?;
        self.eth_address = Some(address);

        Ok(())
    }

    fn read_entropy(&mut self) -> Result<(), KampelaError> {
        if self.entropy.is_some() {
            println!("Entropy read from memory");

            // Derive and cache Ethereum address if we have entropy
            if let Some(ref e) = self.entropy {
                if let Ok(address) = derive_eth_address(e) {
                    self.eth_address = Some(address);
                }
            }
            Ok(())
        } else {
            println!("No entropy found in memory");
            Err(KampelaError::FlashRead)
        }
    }

    fn entropy(&self) -> Result<Vec<u8>, KampelaError> {
        self.entropy.clone().ok_or(KampelaError::FlashRead)
    }

    fn eth_address(&self) -> Result<Address, KampelaError> {
        self.eth_address.ok_or(KampelaError::KeyGeneration)
    }

    fn eth_set_address(&mut self, addr: Address) {
        self.eth_address = Some(addr);
    }

    fn eth_set_transaction(&mut self, transaction: Self::EthTransaction) {
        self.eth_transaction = Some(transaction);
    }

    fn eth_transaction(&self) -> Result<&Self::EthTransaction, KampelaError> {
        self.eth_transaction.as_ref()
            .ok_or_else(|| KampelaError::TransactionInvalid("No transaction".to_string()))
    }

    fn eth_transaction_display(&self) -> Result<String, KampelaError> {
        let tx = self.eth_transaction()?;
        let sender = self.eth_address()?;

        format_eth_transaction_display(tx, sender)
            .map_err(|e| KampelaError::TransactionInvalid(e.to_string()))
    }

    fn eth_sign_transaction(&mut self) -> Result<Vec<u8>, KampelaError> {
        let tx = self.eth_transaction.as_ref()
            .ok_or_else(|| KampelaError::TransactionInvalid("No transaction".to_string()))?;
        let entropy = self.entropy()?;

        let signed_tx = sign_eip1559_transaction(tx, &entropy)?;
        self.signed_tx = Some(signed_tx.clone());

        println!("Transaction signed (simulator)");
        Ok(signed_tx)
    }

}


fn main() {
    let args = Args::parse();
    let init_data_state = AppStateInit::new(args);
    println!("{:?}", init_data_state);

    /*
    // Prepare
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y));
*/
    let mut h = HALHandle::new();
    let desktop = DesktopSimulator::new(&init_data_state);
    let display = SimulatorDisplay::new(SCREEN_SIZE);
    let mut state = UIState::new(desktop, display, &mut h);

    // If transaction was received via NFC, switch to transaction screen
    // If test message was received via NFC, switch to test message screen
    let mut update = if matches!(init_data_state.nfc, NFCState::TestMessage) {
        println!("Test message received - showing test message screen");
        let message = state.platform.test_message.clone().unwrap_or_default();
        state.handle_test_message(message, &mut h)
    } else if matches!(init_data_state.nfc, NFCState::EthTransaction) {
        println!("Eth transaction received - showing transaction screen");
        state.handle_transaction(&mut h)
    } else {
        Some(UpdateRequest::Slow)
    };

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Hello world", &output_settings); //.show_static(&display);

    let mut touches = VecDeque::new();

    // event loop:
    //
    // 1. draw
    // 2. collect input
    // 3. handle input
    // 4. do internal things
    loop {
        // touch event
        if let Some(point) = touches.pop_front() {
            update.propagate(state.handle_tap(point, &mut h));
        };
        // display event; it would be delayed
        if let Some(u) = update.take() {
            sleep(UPDATE_DELAY_TIME);
            let is_clear_update = matches!(u, UpdateRequest::Slow) || matches!(u, UpdateRequest::Fast);
            match state.render(is_clear_update, &mut h) {
                Ok(a) => update.propagate(a),
                Err(e) => println!("{:?}", e),
            };

            match u {
                UpdateRequest::Hidden => {
                    window.update(&state.display);
                    println!("skip {} events in hidden update", window.events().count());
                },
                UpdateRequest::Slow => {
                    invert_display(&mut state.display);
                    window.update(&state.display);
                    sleep(SLOW_UPDATE_TIME);
                    invert_display(&mut state.display);
                    window.update(&state.display);
                    for _i in 0..SLOW_UPDATE_ITER {
                        invert_display(&mut state.display);
                        window.update(&state.display);
                        sleep(BLINK_UPDATE_TIME);
                        invert_display(&mut state.display);
                        window.update(&state.display);
                        sleep(BLINK_UPDATE_TIME);
                    }

                    window.update(&state.display);
                    println!("skip {} events in slow update", window.events().count());
                },
                UpdateRequest::Fast => {
                    invert_display(&mut state.display);
                    window.update(&state.display);
                    sleep(FAST_UPDATE_TIME);
                    invert_display(&mut state.display);
                    window.update(&state.display);
                    println!("fast update");
                },
                UpdateRequest::UltraFast => {
                    window.update(&state.display);
                    println!("ultrafast update");
                    sleep(ULTRAFAST_UPDATE_TIME);
                },
                UpdateRequest::Part(a) => {
                    window.update(&state.display);
                    println!("part update of area {:?}", a);
                    sleep(ULTRAFAST_UPDATE_TIME);
                },
            }
        }
        // this collects ui events, do not remove or simulator will crash
        window.update(&state.display);

        // register input (only pushes are valid in Kampela)
        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    if touches.len() < MAX_TOUCH_QUEUE {
                        touches.push_back(point);
                        println!("point {} registered", point);
                    } else {
                        println!("point {} omitted", point);
                    }
                }
                SimulatorEvent::Quit => return,
                _ => (),
            }
        }

        //and here is some loop time for other things
    }
}

fn invert_display(display: &mut SimulatorDisplay<BinaryColor>) {
    for point in SCREEN_AREA.points() {
        let dot = Pixel::<BinaryColor>(point, display.get_pixel(point).invert());
        dot.draw(display).unwrap();
    };
}
