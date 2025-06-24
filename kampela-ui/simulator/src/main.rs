//! This is simulator to develop Kampela UI mocks
#![deny(unused_crate_dependencies)]
use embedded_graphics_core::{
    pixelcolor::BinaryColor, prelude::Point, primitives::{PointsIter, Rectangle}, Drawable, Pixel
};

use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::{rngs::ThreadRng, thread_rng};
use std::{collections::VecDeque, thread::sleep, time::Duration};
use clap::Parser;
use substrate_crypto_light::sr25519::Public;
use mnemonic_external::{regular::InternalWordList, wordlist, AsWordList, Bits11, WordListElement, WordSet};

/// Amount of time required for full screen update; debounce
///  should be quite large as screen takes this much to clean
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);
const BLINK_UPDATE_TIME: Duration = Duration::new(0, 5000000);
const SLOW_UPDATE_ITER: usize = 8;
const FAST_UPDATE_TIME: Duration = Duration::new(0, 300000000);
const ULTRAFAST_UPDATE_TIME: Duration = Duration::new(0, 300000000);
const UPDATE_DELAY_TIME: Duration = Duration::new(0, 100000000);

const MAX_TOUCH_QUEUE: usize = 2;

use kampela_ui::{
    data_state::{AppStateInit, DataInit, NFCState, StorageState},
    display_def::*,
    platform::{ErrorTransaction, PinCode, Platform},
    uistate::{Event, UIState, UpdateRequest, UpdateRequestMutate},
};

#[derive(Debug)]
pub struct NfcTransactionData {
    pub call: String,
    pub extension: String,
    pub signature: [u8; 130],
}

#[derive(Debug)]
pub struct NfcEthSignRequestData {
    pub request_id: [u8; 16],
    pub sign_data: [u8; 65],
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
    eth_sign_request_data: Option<NfcEthSignRequestData>,
    stored_entropy: Option<Vec<u8>>,
}

impl DesktopSimulator {
    pub fn new(init_state: &AppStateInit) -> Self {
        let pin = [0; 4];
        let (transaction, eth_sign_request_data) = match init_state.nfc {
            NFCState::Empty => (None, None),
            NFCState::Transaction => (Some(NfcTransactionData{
                call: String::from("Hello, this is a transaction!"),
                extension: String::from("Hello, this is a transaction!"),
                signature: [0u8; 130],
            }), None),
            NFCState::EthTransaction => (None, Some(NfcEthSignRequestData{
                request_id: [0u8; 16],
                sign_data: [0u8; 65]
            }))
        };
        let mnemonic = [];
        let wordlist = Self::get_wordlist();
        let stored_entropy = WordSet{
            bits11_set: mnemonic.iter().map(|w| wordlist.bits11_for_word(*w).unwrap()).collect::<Vec<Bits11>>()
        }
            .to_entropy()
            .ok();
        Self {
            pin,
            entropy: None,
            address: None,
            transaction,
            eth_sign_request_data,
            stored_entropy,
        }
    }
}

impl Platform for DesktopSimulator {
    type HAL = HALHandle;
    type Rng<'a> = &'a mut ThreadRng;
    type NfcTransaction = NfcTransactionData;
    type NfcEthSignRequest = NfcEthSignRequestData;
    type AsWordList = InternalWordList;

    fn get_wordlist() -> Self::AsWordList {
        InternalWordList
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

    fn read_entropy(&mut self) -> bool {
        self.entropy = self.stored_entropy.clone();
        println!("entropy read from emulated storage: {:?}", &self.entropy);
        self.entropy.is_some()
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

    fn set_eth_sign_request(&mut self, sign_request: Self::NfcEthSignRequest) {
        self.eth_sign_request_data = Some(sign_request);
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

    fn ethereum(&mut self) -> Option<String> {
        match self.eth_sign_request_data {
            Some(ref a) => {
                let mut out = hex::encode(a.sign_data);
                out.push_str("\n");
                out.push_str(&hex::encode(a.request_id));
                Some(out)
            },
            None => None,
        }
    }

    fn check_address_and_fingerprint_transaction(&self) -> Result<(), ErrorTransaction> {
        Ok(())
    }

    fn signature(&mut self) -> [u8; 130] {
        match self.transaction {
            Some(ref a) => a.signature,
            None =>  panic!("qr not ready!"),
        }
    }

    fn eth_signature(&mut self) -> ([u8; 16], [u8; 65]) {
        match self.eth_sign_request_data {
            Some(ref a) => (a.request_id, a.sign_data),
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
    let mut display = SimulatorDisplay::new(SCREEN_SIZE);
    let mut state = UIState::new(desktop, &mut h);

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
            update.propagate(state.handle_event(Event::Tap(point), &mut h));
        };
        // display event; it would be delayed
        if let Some(u) = update.take() {
            sleep(UPDATE_DELAY_TIME);
            let mut previous = display.clone();
            match state.render(&mut display, &mut h) {
                Ok(a) => update.propagate(a),
                Err(e) => println!("{:?}", e),
            };

            match u {
                UpdateRequest::Invocate => {
                    update.propagate(state.handle_event(Event::Invocation, &mut h));
                    println!("invocation event registered");
                },
                UpdateRequest::Slow => {
                    invert_display(&mut display);
                    window.update(&display);
                    sleep(SLOW_UPDATE_TIME);
                    invert_display(&mut display);
                    window.update(&display);
                    for _i in 0..SLOW_UPDATE_ITER {
                        invert_display(&mut display);
                        window.update(&display);
                        sleep(BLINK_UPDATE_TIME);
                        invert_display(&mut display);
                        window.update(&display);
                        sleep(BLINK_UPDATE_TIME);
                    }

                    window.update(&display);
                    println!("skip {} events in slow update", window.events().count());
                },
                UpdateRequest::Fast => {
                    invert_display(&mut display);
                    window.update(&display);
                    sleep(FAST_UPDATE_TIME);
                    invert_display(&mut display);
                    window.update(&display);
                    println!("fast update");
                },
                UpdateRequest::UltraFast => {
                    window.update(&display);
                    println!("ultrafast update");
                    sleep(ULTRAFAST_UPDATE_TIME);
                },
                UpdateRequest::UltraFastSelective => {
                    draw_selective(&mut previous, &display, None);
                    display = previous;
                    window.update(&display);
                    println!("ultrafast selective update");
                    sleep(ULTRAFAST_UPDATE_TIME);
                },
                UpdateRequest::Part(ref a) => {
                    draw_selective(&mut previous, &display, Some(a));
                    display = previous;
                    window.update(&display);
                    println!("part update with white of area {:?}", a);
                    sleep(ULTRAFAST_UPDATE_TIME);
                },
            }
        }
        // this collects ui events, do not remove or simulator will crash
        //window.update(&display); // removed and didn't crash

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
        let pixel = Pixel::<BinaryColor>(point, display.get_pixel(point).invert());
        pixel.draw(display).unwrap();
    };
}

fn draw_selective(display: &mut SimulatorDisplay<BinaryColor>, new_display: &SimulatorDisplay<BinaryColor>, area: Option<&Rectangle>) {
    // simulate partial write window
    let mut area = area.unwrap_or(&SCREEN_AREA).clone();
    area.top_left.y = if area.top_left.y < 0 {
        0
    } else if area.top_left.y > (SCREEN_SIZE_Y - 1) as i32 {
        (SCREEN_SIZE_Y as i32 / 8 - 1) * 8
    } else {
        (area.top_left.y / 8) * 8
    };

    area.top_left.x = if area.top_left.x < 0 {
        0
    } else if area.top_left.x > (SCREEN_SIZE_X - 1) as i32{
        SCREEN_SIZE_X as i32 - 1
    } else {
        area.top_left.x
    };

    let bottom_right = area.top_left + area.size - Point{x: 1, y: 1};
    
    area.size.height = if bottom_right.y > (SCREEN_SIZE_Y - 1) as i32 {
        (SCREEN_SIZE_Y as u32 / 8) * 8 - area.top_left.y as u32
    } else if bottom_right.y < 0 {
        0
    } else {
        (bottom_right.y as u32 / 8 + 1) * 8 - area.top_left.y as u32
    };

    area.size.width = if bottom_right.x > (SCREEN_SIZE_X - 1) as i32 {
        (SCREEN_SIZE_X as u32) - area.top_left.x as u32
    } else if bottom_right.x < 0 {
        0
    } else {
        (bottom_right.x as u32 + 1) - area.top_left.x as u32
    };
    // simulate selective update mode
    for point in area.points() {
        let pixel = Pixel::<BinaryColor>(point, new_display.get_pixel(point));
        let old_pixel = Pixel::<BinaryColor>(point, display.get_pixel(point));
        if (pixel.1.is_on() && old_pixel.1.is_off()) ||
        (pixel.1.is_off() && old_pixel.1.is_on()) {
            pixel.draw(display).unwrap();
        }
    };
}