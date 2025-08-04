//! Testing basic functionality with `efm32pg23` chip.
//!
//! Operations used here are based on
//!
//! - [reference manual](https://www.silabs.com/documents/public/reference-manuals/efm32pg23-rm.pdf)
//! - [devboard user guide](https://www.silabs.com/documents/public/user-guides/ug515-efm32pg23-brd2504a-user-guide.pdf)
//! - [official API docs](https://docs.silabs.com/gecko-platform/latest/emlib/api/efm32xg23/modules)
//! - [published official open source SDK in C](https://github.com/SiliconLabs/gecko_sdk/tree/gsdk_4.2/platform)

#![no_std]
#![deny(unused_crate_dependencies)]

extern crate alloc;

mod init;
pub mod peripherals;
pub mod devices;
pub mod draw;
pub mod psram_mnemonic;
pub mod debug_display;
pub mod parallel;

use efm32pg23_fix::{Interrupt, NVIC};
use efm32pg23_fix::{CorePeripherals, Peripherals};

use init::init_peripherals;
pub use peripherals::ldma_ch_timer::{CH_TIM0, NFC_BUF_THIRD};

use core::cell::RefCell;
use cortex_m::interrupt::free;
use cortex_m::interrupt::Mutex;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref CORE_PERIPHERALS: Mutex<RefCell<CorePeripherals>> = Mutex::new(RefCell::new({
        let mut core_periph = CorePeripherals::take().unwrap();
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
            NVIC::unmask(Interrupt::SW0);
            NVIC::unmask(Interrupt::IADC);
        }
        core_periph
    }));
    pub static ref PERIPHERALS: Mutex<RefCell<Peripherals>> = Mutex::new(RefCell::new({
        let mut peripherals = Peripherals::take().unwrap();
        init_peripherals(&mut peripherals);
        peripherals
    }));
}

/// Mutexed global access to peripherals
pub fn in_free<F>(mut action: F)
    where F: FnMut(&mut Peripherals)
{
    free(|cs| {
        let mut peripherals = PERIPHERALS.borrow(cs).borrow_mut();
        action(&mut peripherals);
    })
}

/// Mutexed global access to peripherals
pub fn if_in_free<F>(mut action: F) -> bool
    where F: FnMut(&mut Peripherals) -> bool
{
    free(|cs| {
        let mut peripherals = PERIPHERALS.borrow(cs).borrow_mut();
        action(&mut peripherals)
    })
}

#[derive(Debug, PartialEq)]
pub enum FreeError {
    MutexLocked,
}
