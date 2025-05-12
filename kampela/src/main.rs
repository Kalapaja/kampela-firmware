#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![deny(unused_crate_dependencies)]

extern crate alloc;
extern crate core;

use alloc::{borrow::ToOwned, boxed::Box, format};
use core::{alloc::Layout, panic::PanicInfo};
use cortex_m::interrupt::free;
use cortex_m_rt::{entry, exception, ExceptionFrame};

use embedded_alloc::Heap;

use kampela_system::{
    debug_display::burning_tank, devices::{
        flash::{flash_sleep, flash_wait_ready, flash_wakeup},
        power::ADC,
        psram::psram_reset,
        touch::{clear_touch_if, enable_touch_int, is_touch_int, Read, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}
    }, if_in_free, in_free, parallel::{AsyncOperation, Threads}, peripherals::{ldma::LdmaCh, ldma_ch_usart_rx::{FlashPSRamCopy, LDMAchUSART0Rx, ReceivableUSART}}, CORE_PERIPHERALS, PERIPHERALS
};
use efm32pg23_fix::{Interrupt, interrupt, Peripherals, NVIC, SYST};

mod ui;
use ui::UI;
mod hardware;
mod nfc;
use nfc::{NfcError, NfcReceiver, NfcResult, NfcStateOutput};
mod touch;
use touch::Touches;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use core::mem::MaybeUninit;
const HEAP_SIZE: usize = 0x6500;
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

unsafe fn init_heap() {
    HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE)
}

/*
static mut GPIO_ODD_INT: bool = false;
static mut COUNT_ODD: bool = false;
static mut GPIO_EVEN_INT: bool = false;
static mut COUNT_EVEN: bool = false;
static mut READER: Option<[u8;5]> = None;
*/

#[alloc_error_handler]
fn oom(l: Layout) -> ! {
    panic!("out of memory: {:?}, heap used: {}, free: {}", l, HEAP.used(), HEAP.free());
}

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    let mut peripherals = unsafe{Peripherals::steal()};
    unsafe { init_heap(); } // free up heap for critical drawing buffer
    burning_tank(&mut peripherals, format!("{:?}", panic));
    loop {}
}

#[exception]
unsafe fn HardFault(exception_frame: &ExceptionFrame) -> ! {
    panic!("hard fault: {:?}", exception_frame)
}

#[entry]
fn main() -> ! {
    unsafe { init_heap(); }
    
    free(|cs| {
        PERIPHERALS.borrow(cs); //will init peripheral
        let mut core_periph = CORE_PERIPHERALS.borrow(cs).borrow_mut();
        // Errata CUR_E302 fix
        // enable FPU to reduce power consumption in EM1
        unsafe {
            core_periph.SCB.cpacr.modify(|w_reg| w_reg | (3 << 20) | (3 << 22));
        }

        NVIC::unpend(Interrupt::LDMA);
        NVIC::mask(Interrupt::LDMA);
        unsafe {
            core_periph.NVIC.set_priority(Interrupt::LDMA, 3);
            NVIC::unmask(Interrupt::LDMA);
        }
    });

    in_free(|peripherals| {
        flash_wakeup(peripherals);
        flash_wait_ready(peripherals);
        psram_reset(peripherals);
    });
    LDMAchUSART0Rx::set_static_cell(Some(ReceivableUSART::FlashPSRamCopy(FlashPSRamCopy::new())));
    let mut counter = 0;
    while !LDMAchUSART0Rx::done() {
    }
    LDMAchUSART0Rx::take_static_cell();
    in_free(|peripherals| {
        flash_sleep(peripherals);
    });
    //let pair_derived = Keypair::from_bytes(ALICE_KAMPELA_KEY).unwrap();

    // Development: erase seed when Pilkki can't
  
/*
    in_free(|peripherals| {
            flash_wakeup(peripherals);

            flash_unlock(peripherals);
            flash_erase_page(peripherals, 0);
            flash_wait_ready(peripherals);
    });
*/

    // hard derivation
    //let junction = DeriveJunction::hard("kampela");
    // let pair_derived = pair
    //         //.hard_derive_mini_secret_key(Some(ChainCode(*junction.inner())), b"")
    //         .0
    //         .expand_to_keypair(ExpansionMode::Ed25519);
            // initialize SYST for Timer
    free(|cs| {  
        let mut core_periph = CORE_PERIPHERALS.borrow(cs).borrow_mut();
        core_periph.SYST.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        core_periph.SYST.set_reload(SYST::get_ticks_per_10ms());
        core_periph.SYST.clear_current();
        core_periph.SYST.enable_counter();
    });
    
    let mut main_state = MainState::new(());//&nfc_buffer);
    loop {
        main_state.advance(());
    }
}

enum MainStatus {
    ADCProbe,
    NFCRead(NfcReceiver),
    Display(Box<UI>),
    TouchRead(Option<Read<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>>),
}

impl Default for MainStatus {
    fn default() -> Self {
        MainStatus::ADCProbe
    }
}

struct MainState {
    threads: Threads<MainStatus, 3>,
    adc: ADC,
    ui: Option<Box<UI>>,
    touches: Touches,
}

impl AsyncOperation for MainState {
    type Init = ();//&'a [u16; 3*BUF_THIRD];
    type Input<'b> = ();
    type Output = ();
    /// Start of UI.
    fn new(_: Self::Init) -> Self {
        let ui = UI::new(());
        clear_touch_if();

        return Self {
            threads: Threads::from([
                MainStatus::ADCProbe,
                MainStatus::NFCRead(NfcReceiver::new()),
                //MainStatus::TouchRead(None),
                //MainStatus::Display(None), // PixelBuffer is boxed in FrameBuffer
            ]),
            adc: ADC::new(()),
            ui: Some(Box::new(ui)),
            touches: Touches::new()
        }
    }

    /// Call in event loop to progress through Kampela states
    fn advance(&mut self, _: ()) {
        match self.threads.turn() {
            MainStatus::ADCProbe => {
                self.adc.advance(());
            },
            MainStatus::NFCRead(receiver) => {
                if let Some(s) = receiver.advance(self.adc.read()) {
                    match s {
                        Err(e) => {
                            match e {
                                NfcError::InvalidAddress => {
                                    if let Some(ref mut u) = self.ui {
                                        u.handle_message("Invalid sender address".to_owned())
                                    }
                                }
                            }
                            if let Some(u) = self.ui.take() {
                                self.threads.change(MainStatus::Display(u));
                            }
                        }
                        Ok(s) => {
                            match s {
                                NfcStateOutput::Operational(i) => {
                                    if i == 1 {
                                        if let Some(ref mut u) = self.ui {
                                            u.handle_message("Receiving NFC packets...".to_owned());
                                        }
                                        if !self.threads.is_all_running(&[
                                            |s| matches!(s, MainStatus::Display(..))
                                        ]) {
                                            if let Some(u) = self.ui.take() {
                                                self.threads.wind(MainStatus::Display(u));
                                            }
                                        };
                                    }
                                },
                                NfcStateOutput::Done(r) => {
                                    match r {
                                        NfcResult::Empty => {
                                            if !self.threads.is_all_running(&[
                                                |s| matches!(s, MainStatus::Display(..))
                                            ]) {
                                                if let Some(u) = self.ui.take() {
                                                    self.threads.wind(MainStatus::Display(u));
                                                }
                                            };
                                        },
                                        NfcResult::DisplayAddress => {
                                            self.threads.try_change_any(|status| {
                                                if let MainStatus::Display(ui) = status {
                                                    ui.handle_address([0;76]);
                                                }
                                            });
                                        },
                                        NfcResult::Transaction(transaction) => {
                                            self.threads.try_change_any(|status| {
                                                if let MainStatus::Display(ui) = status {
                                                    ui.handle_transaction(transaction.clone());
                                                }
                                            });
                                        }
                                    }
                                    enable_touch_int();
                                    self.threads.change(MainStatus::TouchRead(None));
                                }
                            }
                        }
                    }
                }
            },
            MainStatus::Display(ui) => {
                if ui.advance((self.adc.read(), &mut self.touches)) == Some(false) {
                    self.threads.hold();
                }
            },
            MainStatus::TouchRead(state) => {
                match state {
                    None => {
                        if is_touch_int() {
                            self.threads.change(MainStatus::TouchRead(Some(Read::new(()))));
                        }
                    },
                    Some(reader) => {
                        match reader.advance(()) {
                            Ok(Some(Some(touch))) => {
                                self.touches.try_push_touch_data(touch);
                                self.threads.change(MainStatus::TouchRead(None));
                            },
                            Ok(Some(None)) => {self.threads.hold()},
                            Ok(None) => {}
                            Err(e) => panic!("{:?}", e),
                        }
                    }
                }
            },
        }
    }
}
