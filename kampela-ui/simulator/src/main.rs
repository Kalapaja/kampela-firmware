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
use std::{collections::VecDeque, thread::sleep, time::Duration};
use clap::Parser;
use substrate_crypto_light::sr25519::Public;
use mnemonic_external::regular::InternalWordList;

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
    platform::{PinCode, Platform},
    uistate::{UIState, UpdateRequest, UpdateRequestMutate},
};

#[derive(Debug)]
pub struct NfcTransactionData {
    pub call: String,
    pub extension: String,
    pub signature: [u8; 130],
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'I')]
    key_was_created: bool,

    #[arg(short = 'T')]
    transaction_received: bool,
}

impl DataInit<Args> for AppStateInit {
    fn new(params: Args) -> AppStateInit {
        let storage = StorageState {
            key_created: params.key_was_created,
        };

        let nfc = if params.transaction_received {
            NFCState::Transaction
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
    address: Option<[u8; 76]>,
    transaction: Option<NfcTransactionData>,
    stored_entropy: Option<Vec<u8>>,
}

impl DesktopSimulator {
    pub fn new(init_state: &AppStateInit) -> Self {
        let pin = [0; 4];
        let transaction = match init_state.nfc {
            NFCState::Empty => None,
            NFCState::Transaction => Some(NfcTransactionData{
                call: String::from("Hello, this is a transaction!"),
                extension: String::from("Hello, this is a transaction!"),
                signature: [0u8; 130],
            }),
        };
        Self {
            pin,
            entropy: None,
            address: None,
            transaction: transaction,
            stored_entropy: None,
        }
    }
}

impl Platform for DesktopSimulator {
    type HAL = HALHandle;
    type Rng<'a> = &'a mut ThreadRng;
    type NfcTransaction = NfcTransactionData;
    type AsWordList = InternalWordList;

    fn get_wordlist<'a>() -> &'a Self::AsWordList {
        &InternalWordList
    }

    fn rng<'a>(h: &'a mut Self::HAL) -> Self::Rng<'a> {
        &mut h.rng
    }

    fn pin(&self) -> &PinCode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut PinCode {
        &mut self.pin
    }

    fn store_entropy(&mut self, e: &[u8]) {
        self.entropy = Some(e.to_vec());
        println!("entropy stored (not really, this is emulator)");
    }

    fn read_entropy(&mut self) {
        self.entropy = self.stored_entropy.clone();
        println!("entropy read from emulated storage: {:?}", &self.entropy);
    }

    fn public(&self) -> Option<Public> {
        self.pair().map(|pair| pair.public())
    }

    fn entropy(&self) -> Option<Vec<u8>> {
        self.entropy.clone()
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_transaction(&mut self, transaction: Self::NfcTransaction) {
        self.transaction = Some(transaction);
    }

    fn call(&mut self) -> Option<String> {
        match self.transaction {
            Some(ref a) => Some(a.call.to_owned()),
            None => None,
        }
    }

    fn extensions(&mut self) -> Option<String> {
        match self.transaction {
            Some(ref a) => Some(a.extension.to_owned()),
            None => None,
        }
    }

    fn signature(&mut self) -> [u8; 130] {
        match self.transaction {
            Some(ref a) => a.signature,
            None =>  panic!("qr not ready!"),
        }
    }

    fn address(&mut self) -> &[u8; 76] {
        if let Some(ref a) = self.address {
            a
        } else {
            panic!("address qr not ready!");
        }
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

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Hello world", &output_settings); //.show_static(&display);
    
    let mut update = Some(UpdateRequest::Slow);

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