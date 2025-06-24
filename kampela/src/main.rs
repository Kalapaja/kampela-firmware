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
        flash::{init_flash_copy_to_psram, wait_flash_copy_to_psram}, power::{voltage, wait_for_energy}, touch::{clear_touch_if, enable_touch_int}
    }, parallel::{AsyncOperation, Threads}, peripherals::{adc::adc_cmp_ien, ldma_ch_timer::purge_ldma_nfc_buffers}, CORE_PERIPHERALS, PERIPHERALS
};
use efm32pg23_fix::{Interrupt, Peripherals, NVIC, SYST};

mod ui;
use ui::UI;
mod hardware;
mod nfc;
use nfc::{NfcError, NfcReceiver, NfcResult, NfcState};
mod touch;

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
        NVIC::unpend(Interrupt::GPIO_EVEN);
        NVIC::mask(Interrupt::GPIO_EVEN);
        NVIC::unpend(Interrupt::TIMER2);
        NVIC::mask(Interrupt::TIMER2);
        NVIC::unpend(Interrupt::IADC);
        NVIC::mask(Interrupt::IADC);
        unsafe {
            core_periph.NVIC.set_priority(Interrupt::LDMA, 3);
            core_periph.NVIC.set_priority(Interrupt::GPIO_EVEN, 5);
            core_periph.NVIC.set_priority(Interrupt::TIMER2, 4);
            core_periph.NVIC.set_priority(Interrupt::SW0, 6);
            core_periph.NVIC.set_priority(Interrupt::IADC, 7);
            NVIC::unmask(Interrupt::LDMA);
            NVIC::unmask(Interrupt::GPIO_EVEN);
            NVIC::unmask(Interrupt::TIMER2);
            NVIC::unmask(Interrupt::SW0);
            NVIC::unmask(Interrupt::IADC);
        }
    });

    wait_for_energy();
    //flash_copy_to_psram();
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
    Init(Option<()>),
    NFCRead(NfcReceiver),
    Display(UI)
}

impl Default for MainStatus {
    fn default() -> Self {
        MainStatus::Init(None)
    }
}

struct MainState {
    threads: Threads<MainStatus, 2>,
}

impl AsyncOperation for MainState {
    type Init = ();//&'a [u16; 3*BUF_THIRD];
    type Input<'b> = ();
    type Output = ();
    /// Start of UI.
    fn new(_: Self::Init) -> Self {
        clear_touch_if();
        enable_touch_int();
        return Self {
            threads: Threads::new(MainStatus::Init(None)),
        }
    }

    /// Call in event loop to progress through Kampela states
    fn advance(&mut self, _: ()) {
        match self.threads.turn() {
            MainStatus::Init(state) => {
                match state {
                    None => {
                        *state = Some(());
                        init_flash_copy_to_psram();
                        self.threads.wind(MainStatus::NFCRead(NfcReceiver::new()));
                    },
                    Some(_) => {
                        if wait_flash_copy_to_psram() { return }
                        self.threads.switch(MainStatus::Display(UI::new(())));
                    }
                }

            },
            MainStatus::NFCRead(receiver) => {
                match receiver.advance() {
                    Err(e) => {
                        match e {
                            NfcError::InvalidAddress => {
                                self.threads.try_change_any(|status| {
                                    if let MainStatus::Display(ui) = status {
                                        ui.handle_message("Invalid sender address".to_owned());
                                    }
                                });
                            }
                        }
                        self.threads.sync();
                    }
                    Ok(s) => {
                        match s {
                            NfcState::Operational(i) => {
                                if i > 0 {
                                    self.threads.try_change_any(|status| {
                                        if let MainStatus::Display(ui) = status {
                                            ui.handle_message("Receiving NFC packets...".to_owned());
                                        }
                                    });
                                }
                            },
                            NfcState::Done(r) => {
                                match r {
                                    NfcResult::Empty => {
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
                                    },
                                    NfcResult::EthSignRequest(eth_sign_request) => {
                                        let mut success = false;
                                        self.threads.try_change_any(|status| {
                                            if let MainStatus::Display(ui) = status {
                                                if ui.update_is_pending() { return }
                                                ui.handle_eth_sign_request(eth_sign_request.clone());
                                                success = true;
                                            }
                                        });
                                        if !success { return }
                                    }
                                }
                                self.threads.sync();
                            }
                        }
                    }
                }
            },
            MainStatus::Display(ui) => {
                if ui.advance(()) == Some(false) {
                    self.threads.hold();
                }
            },
        }
    }
}
