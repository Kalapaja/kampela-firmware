//! This is simulator to develop Kampela UI mocks
#![cfg(feature="std")]
use embedded_graphics_core::{
    geometry::Size,
    pixelcolor::BinaryColor,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::{rngs::ThreadRng, thread_rng};
use std::{thread::sleep, time::Duration};
use clap::Parser;

#[macro_use]
extern crate lazy_static;

/// Amount of time required for full screen update; debounce
///  should be quite large as screen takes this much to clean
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);

pub mod display_def;
pub use display_def::*;

mod platform;
use platform::{public_from_entropy, PinCode, Platform};

mod pin {
    pub mod pin;
    pub mod pindots;
    pub mod pinpad;
    pub mod pinbutton;
}

mod restore_or_generate;
pub mod seed_entry{
    pub mod seed_entry;
    pub mod entry;
    pub mod proposal;
    pub mod phrase;
    pub mod keyboard;
    pub mod key;
}

pub mod nav_bar{
    pub mod nav_bar;
    pub mod nav_button;
}
mod widget {
    pub mod view;
}

mod backup;

mod uistate;
use uistate::UIState;

mod data_state;
use data_state::{AppStateInit, NFCState, DataInit, StorageState};

mod transaction;
mod message;
mod qr;

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
    display: SimulatorDisplay<BinaryColor>,
    entropy: Option<Vec<u8>>,
    address: Option<[u8; 76]>,
    transaction: Option<NfcTransactionData>,
    stored_entropy: Option<Vec<u8>>,
}

impl DesktopSimulator {
    pub fn new(init_state: &AppStateInit, h: &mut HALHandle) -> Self {
        let pin = [0; 4]; //TODO proper pin initialization
        let display = SimulatorDisplay::new(Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y));
        let transaction = match init_state.nfc {
            NFCState::Empty => None,
            NFCState::Transaction => Some(NfcTransactionData{
                call: String::from("Hello, this is a transaction!"),
                extension: String::from("Hello, this is a transaction!"),
                signature: [0u8; 130],
            }),
        };
        Self {
            pin: pin,
            display: display,
            entropy: None,
            address: None,
            transaction: transaction,
            stored_entropy: None,
        }
    }
}

impl Platform for DesktopSimulator {
    type HAL = HALHandle;
    type Rng = ThreadRng;
    type Display = SimulatorDisplay<BinaryColor>;
    type NfcTransaction = NfcTransactionData;

    fn rng<'a>(h: &'a mut Self::HAL) -> &'a mut Self::Rng {
        &mut h.rng
    }

    fn pin(&self) -> &PinCode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut PinCode {
        &mut self.pin
    }

    fn display(&mut self) -> &mut Self::Display {
        &mut self.display
    }

    fn store_entropy(&mut self, e: &[u8]) {
        self.entropy = Some(e.to_vec());
        println!("entropy stored (not really, this is emulator)");
    }

    fn read_entropy(&mut self) {
        self.entropy = self.stored_entropy.clone();
        println!("entropy read from emulated storage: {:?}", &self.entropy);
    }

    fn entropy(&self) -> Option<Vec<u8>> {
        self.entropy.clone()
    }

    fn public(&self) -> Option<[u8; 32]> {
        match &self.entropy {
            Some(e) => public_from_entropy(e),
            None => None,
        }
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_transaction(&mut self, transaction: Self::NfcTransaction) {
        self.transaction = Some(transaction);
    }

    fn call(&mut self) -> Option<(String, &mut Self::Display)> {
        match self.transaction {
            Some(ref a) => Some((a.call.to_owned(), &mut self.display)),
            None => None,
        }
    }

    fn extensions(&mut self) -> Option<(String, &mut Self::Display)> {
        match self.transaction {
            Some(ref a) => Some((a.extension.to_owned(), &mut self.display)),
            None => None,
        }
    }

    fn signature(&mut self, h: &mut Self::HAL) -> ([u8; 130], &mut Self::Display) {
        match self.transaction {
            Some(ref a) => (a.signature, &mut self.display),
            None =>  panic!("qr not ready!"),
        }
    }

    fn address(&mut self) -> (&[u8; 76], &mut Self::Display) {
        if let Some(ref a) = self.address {
            (a, &mut self.display)
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
    let desktop = DesktopSimulator::new(&init_data_state, &mut h);

    let mut state = UIState::new(desktop, &mut h);

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Hello world", &output_settings); //.show_static(&display);
    
    let mut update = uistate::UpdateRequest::new();
    update.set_slow();

    // event loop:
    //
    // 1. draw
    // 2. collect input
    // 3. handle input
    // 4. do internal things
    loop {
        // display event; it would be delayed
        let f = update.read_fast();
        let s = update.read_slow();
        let p = update.read_part();
        let uf = update.read_ultrafast();
        let i = update.read_hidden();

        if i || f || s || p.is_some() || uf {
            match state.render::<SimulatorDisplay<BinaryColor>>(f || s, &mut h) {
                Ok(u) => update = u,
                Err(e) => println!("{:?}", e),
            };
        }

        if i {
            window.update(state.display());
            println!("skip {} events in hidden update", window.events().count());
            //no-op for non-EPD
        }

        if f {
            window.update(state.display());
            println!("skip {} events in fast update", window.events().count());
            //no-op for non-EPD
        }

        if p.is_some() {
            window.update(state.display());
            println!("skip {} events in part update", window.events().count());
            //no-op for non-EPD
        }

        if uf {
            window.update(state.display());
            println!("skip {} events in ultrafast update", window.events().count());
            //no-op for non-EPD
        }

        if s {
            sleep(SLOW_UPDATE_TIME);
            window.update(state.display());
            println!("skip {} events in slow update", window.events().count());
        }

        // this collects ui events, do not remove or simulator will crash
        window.update(state.display());

        // handle input (only pushes are valid in Kampela)
        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    println!("{}", point);
                        match state.handle_tap::<SimulatorDisplay<BinaryColor>>(point, &mut h) {
                            Ok(a) => update = a,
                            Err(e) => println!("{e}"),
                        };
                }
                SimulatorEvent::Quit => return,
                _ => (),
            }
        }

        //and here is some loop time for other things
    }
}
