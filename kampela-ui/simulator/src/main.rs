//! This is simulator to develop Kampela UI mocks
#![deny(unused_crate_dependencies)]
use embedded_graphics_core::{pixelcolor::BinaryColor, primitives::PointsIter, Drawable, Pixel};

use clap::Parser;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
// use mnemonic_external::regular::InternalWordList;
use rand::{rngs::ThreadRng, thread_rng};
use std::{collections::VecDeque, thread::sleep, time::Duration};
use substrate_crypto_light::sr25519::Public;

/// Amount of time required for full screen update; debounce
///  should be quite large as screen takes this much to clean
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);
const BLINK_UPDATE_TIME: Duration = Duration::new(0, 5000000);
const SLOW_UPDATE_ITER: usize = 8;
const FAST_UPDATE_TIME: Duration = Duration::new(1, 0);
const ULTRAFAST_UPDATE_TIME: Duration = Duration::new(1, 0);
const UPDATE_DELAY_TIME: Duration = Duration::new(0, 500000000);

const MAX_TOUCH_QUEUE: usize = 2;

mod infernal_wordlist;
use infernal_wordlist::InfernalWordList;

mod flash_emulation;
use flash_emulation::FlashData;

use mnemonic_external::AsWordList;
use mnemonic_external::Bits11;
use mnemonic_external::WordSet;

use kampela_ui::{
    data_state::{AppStateInit, DataInit, NFCState, StorageState},
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

    #[arg(short, long)]
    wordlist_path: Option<String>,

    #[arg(short, long)]
    flash_path: Option<String>,
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
        Self { rng: rng }
    }
}

#[derive(Debug)]
struct DesktopSimulator {
    pin: PinCode,
    entropy: Option<Vec<u8>>,
    address: Option<[u8; 76]>,
    transaction: Option<NfcTransactionData>,
    flash_data: Option<FlashData>,
}

impl DesktopSimulator {
    pub fn new(init_state: &AppStateInit) -> Self {
        let args = Args::parse();
        let mut flash_data: Option<FlashData> = None;
        if let Some(path) = args.flash_path {
            flash_data = Some(FlashData::from_json_file(&path));
        }
        let mut pin = [0; 4];
        if let Some(flash_data) = &flash_data {
            if let Some(stored_pin) = flash_data.pin {
                pin = stored_pin;
                println!("pin read from emulated storage: {:?}", pin);
            }
        }

        let transaction = match init_state.nfc {
            NFCState::Empty => None,
            NFCState::Transaction => Some(NfcTransactionData {
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
            flash_data: flash_data,
        }
    }
}

impl Platform for DesktopSimulator {
    type HAL = HALHandle;
    type Rng<'a> = &'a mut ThreadRng;
    type NfcTransaction = NfcTransactionData;
    type AsWordList = InfernalWordList;

    fn get_wordlist() -> Self::AsWordList {
        let args = Args::parse();
        if let Some(path) = args.wordlist_path {
            return InfernalWordList::from_file(&path);
        }
        return InfernalWordList::new();
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
        println!("Store entropy: {} bytes {:?}", e.len(), e);
        let wordset = WordSet::from_entropy(e).unwrap();
        let wordlist = Self::get_wordlist();
        println!("Wordset is: {:?}", wordset.to_phrase(&wordlist).unwrap());
        let args = Args::parse();

        if let Some(path) = args.flash_path {
            let mut flash_data = FlashData::from_json_file(&path);
            let mut actual_entropy: [u8; 32] = [0; 32];
            actual_entropy.copy_from_slice(e);
            flash_data.entropy = Some(actual_entropy);
            flash_data.to_json_file(&path);
            self.flash_data = Some(flash_data);
            println!("Flash file ({}) updated with entropy", path);
        }

        self.entropy = Some(e.to_vec());
    }

    fn read_entropy(&mut self) {
        if self.flash_data.is_none() {
            println!("No flash data");
            return;
        }
        if self.flash_data.as_ref().unwrap().entropy.is_none() {
            println!("No entropy in flash data");
            return;
        }
        let flash_data: &FlashData = self.flash_data.as_ref().unwrap();
        self.entropy = Some(flash_data.entropy.unwrap().to_vec());
        let wordlist = Self::get_wordlist();
        let entropy = self.entropy.as_ref().unwrap();
        let wordset = WordSet::from_entropy(entropy).unwrap();
        println!(
            "entropy read from emulated storage: {:?}",
            &self.entropy.as_ref().unwrap()
        );
        println!("Wordset is: {:?}", wordset.to_phrase(&wordlist).unwrap());
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
            None => panic!("qr not ready!"),
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
    let mut window = Window::new("Kampela Emulator", &output_settings); //.show_static(&display);

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
            let is_clear_update =
                matches!(u, UpdateRequest::Slow) || matches!(u, UpdateRequest::Fast);
            match state.render(is_clear_update, &mut h) {
                Ok(a) => update.propagate(a),
                Err(e) => println!("{:?}", e),
            };

            match u {
                UpdateRequest::Hidden => {
                    window.update(&state.display);
                    println!("skip {} events in hidden update", window.events().count());
                }
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
                }
                UpdateRequest::Fast => {
                    invert_display(&mut state.display);
                    window.update(&state.display);
                    sleep(FAST_UPDATE_TIME);
                    invert_display(&mut state.display);
                    window.update(&state.display);
                    println!("fast update");
                }
                UpdateRequest::UltraFast => {
                    window.update(&state.display);
                    println!("ultrafast update");
                    sleep(ULTRAFAST_UPDATE_TIME);
                }
                UpdateRequest::Part(a) => {
                    window.update(&state.display);
                    println!("part update of area {:?}", a);
                    sleep(ULTRAFAST_UPDATE_TIME);
                }
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
    }
}
