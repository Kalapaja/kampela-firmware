//! This is simulator to develop Kampela UI mocks
#![deny(unused_crate_dependencies)]
use embedded_graphics_core::{
    pixelcolor::BinaryColor, prelude::Point, primitives::{PointsIter, Rectangle}, Drawable, Pixel
};

use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::{rngs::ThreadRng, thread_rng};
use uuid::Uuid;
use std::{collections::VecDeque, thread::sleep, time::Duration};
use clap::Parser;
use mnemonic_external::{regular::InternalWordList, AsWordList, Bits11, WordSet};

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
    data_state::{AppStateInit, DataInit, NFCState, StorageState}, display_def::*, ethereum_decode::{EthSignRequest, SignDataType}, messages::{EthSignRequestDecodeError}, platform::{PinCode, Platform}, uistate::{Event, UIState, UpdateRequest, UpdateRequestMutate}
};

#[derive(Debug, Clone)]
pub struct SubstrateTransactionData {
    pub call: String,
    pub extension: String,
    pub signature: [u8; 130],
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'I')]
    key_was_created: bool,

    #[arg(short = 'T', default_value = "0")]
    transaction_received: usize,
}

impl DataInit<Args> for AppStateInit {
    fn new(params: Args) -> AppStateInit {
        let storage = StorageState {
            key_created: params.key_was_created,
        };

        let nfc = match params.transaction_received {
            2 => NFCState::EthEip712Transaction,
            3 => NFCState::EthTypedTransaction,
            _ => NFCState::Empty
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

#[derive(Clone)]
enum Transaction {
    Ethereum(EthSignRequest)
}

struct DesktopSimulator {
    pin: PinCode,
    seed: Option<[u8; 64]>,
    address: Option<[u8; 76]>,
    transaction: Option<Transaction>,
    stored_seed: Option<[u8; 64]>,
}

impl DesktopSimulator {
    pub fn new(init_state: &AppStateInit) -> Self {
        let pin = [0; 4];
        let transaction = match init_state.nfc {
            NFCState::Empty => None,
            NFCState::EthEip712Transaction => {
                Some(Transaction::Ethereum(EthSignRequest{
                    request_id: Some(Uuid::from_slice(&hex::decode("7f820ab5d08049a497dc4d0cf754184b").unwrap()).unwrap()),
                    sign_data: hex::decode("7b227479706573223a7b22454950373132446f6d61696e223a5b7b226e616d65223a226e616d65222c2274797065223a22737472696e67227d2c7b226e616d65223a2276657273696f6e222c2274797065223a22737472696e67227d2c7b226e616d65223a22636861696e4964222c2274797065223a2275696e74323536227d2c7b226e616d65223a22766572696679696e67436f6e7472616374222c2274797065223a2261646472657373227d5d2c225472616e7366657252657175657374223a5b7b226e616d65223a22746f222c2274797065223a2261646472657373227d2c7b226e616d65223a22616d6f756e74222c2274797065223a2275696e74323536227d5d7d2c227072696d61727954797065223a225472616e7366657252657175657374222c22646f6d61696e223a7b226e616d65223a224b61726d612052657175657374222c2276657273696f6e223a2231222c22636861696e4964223a223078616133366137222c22766572696679696e67436f6e7472616374223a22307843634343636363634343434363434343434343634363436363436343434363436363636363636343227d2c226d657373616765223a7b22746f223a22307843634343636363634343434363434343434343634363436363436343434363436363636363636343222c22616d6f756e74223a313233347d7d").unwrap(),
                    data_type: SignDataType::EthTypedData,
                    chain_id: 111551111,
                    derivation_path: "44'/60'/0'/0/0".to_string(),
                    source_fingerprint: [0x73, 0xc5, 0xda, 0x0a],
                    address: Some(alloy_primitives::Address::from_slice(&hex::decode("9858EffD232B4033E47d90003D41EC34EcaEda94").unwrap())),
                    origin: None
                }))
            },
            NFCState::EthTypedTransaction => Some(Transaction::Ethereum(EthSignRequest{
                request_id: Some(Uuid::from_slice(&hex::decode("e5cb0877e2c64bb89ced485721d0a24b").unwrap()).unwrap()),
                sign_data: hex::decode("02e383aa36a781d18080828e59949858effd232b4033e47d90003d41ec34ecaeda948080c0").unwrap(),
                data_type: SignDataType::EthTypedTransaction,
                chain_id: 111551111,
                derivation_path: "44'/60'/0'/0/0".to_string(),
                source_fingerprint: [0x73, 0xc5, 0xda, 0x0a],
                address: Some(alloy_primitives::Address::from_slice(&hex::decode("9858EffD232B4033E47d90003D41EC34EcaEda94").unwrap())),
                origin: None
            }))
        };
        let mnemonic = ["abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "about"];
        
        let wordlist = Self::get_wordlist();
        let stored_seed = if init_state.storage.key_created {
            let e = WordSet{
                bits11_set: mnemonic.iter().map(|w| wordlist.bits11_for_word(*w).unwrap()).collect::<Vec<Bits11>>()
            }
                .to_entropy()
                .ok();
            e.map(|e| Self::from_entropy_to_eth_seed(&e))
        } else {
            None
        };

        Self {
            pin,
            seed: None,
            address: None,
            transaction,
            stored_seed,
        }
    }
}

impl Platform for DesktopSimulator {
    type HAL = HALHandle;
    type Rng<'a> = &'a mut ThreadRng;
    type NfcEthSignRequest = EthSignRequest;
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

    fn store_seed(&mut self, e: &[u8]) {
        let seed = Self::from_entropy_to_eth_seed(e);
        self.stored_seed = Some(seed);
        println!("seed stored (not really, this is emulator)");
    }

    fn read_seed(&mut self) -> bool {
        self.seed = self.stored_seed.clone();
        println!("entropy read from emulated storage: {:?}", &self.seed);
        self.seed.is_some()
    }

    fn seed(&self) -> Option<[u8; 64]> {
        self.seed.clone()
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_eth_sign_request(&mut self, sign_request: Self::NfcEthSignRequest) {
        self.transaction = Some(Transaction::Ethereum(sign_request));
    }

    fn eth_sign_request(&self) -> Result<EthSignRequest, EthSignRequestDecodeError> {
        match self.transaction {
            Some(Transaction::Ethereum(ref a)) => {
                Ok(a.clone())
            },
            _ => Err(EthSignRequestDecodeError::NoEthSignRequestDataStored),
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
    let mut transaction = desktop.transaction.clone();
    let mut state = UIState::new(desktop, &mut h);

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Hello world", &output_settings); //.show_static(&display);
    
    let mut update = Some(UpdateRequest::Fast);
    
    if transaction.is_some() {
        update = state.handle_message("Receiving Nfc package".to_string(), &mut h);
    }

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

            match transaction {
                Some(Transaction::Ethereum(_)) => {
                    update = state.handle_eth_sign_request();
                    transaction = None;
                },
                None => ()
            }

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
                    draw_selective(&mut previous, &display, &Vec::new());
                    display = previous;
                    window.update(&display);
                    println!("ultrafast selective update");
                    sleep(ULTRAFAST_UPDATE_TIME);
                },
                UpdateRequest::Part(ref a) => {
                    draw_selective(&mut previous, &display, a);
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

fn draw_selective(display: &mut SimulatorDisplay<BinaryColor>, new_display: &SimulatorDisplay<BinaryColor>, areas: &Vec<Rectangle>) {
    if areas.is_empty() {
        draw_area(display, new_display, SCREEN_AREA);
        return
    }
    for area in areas.iter() {
        draw_area(display, new_display, *area)
    }
}

fn draw_area(display: &mut SimulatorDisplay<BinaryColor>, new_display: &SimulatorDisplay<BinaryColor>, mut area: Rectangle) {
    // simulate partial write window
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