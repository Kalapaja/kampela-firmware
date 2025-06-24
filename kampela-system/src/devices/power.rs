//! Power measurement unit
use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use efm32pg23_fix::{interrupt, Interrupt, NVIC, SCB};

use crate::{free, if_in_free, in_free, peripherals::adc::{adc_cmp_ien, adc_icmp, read_adc}, CORE_PERIPHERALS};

const THRESHOLD_VOLTAGE: u16 = 4000;
const INIT_VOLTAGE_THRESHOLD: u16 = 8000;
const CUT_OUT_VOLTAGE: u16 = 100;

static POWER_STATE: Mutex<RefCell<PowerState>> = Mutex::new(RefCell::new(PowerState::Saving));

enum PowerState {
    Saving,
    Active,
}

#[interrupt]
fn IADC() {
    if voltage() <= CUT_OUT_VOLTAGE as i32 {
        SCB::sys_reset();
    }
    if if_in_free(|peripherals| {
        if peripherals.iadc0_s.if_().read().scancmp().bit_is_set() {
            peripherals.iadc0_s.if_clr().write(|w_reg| w_reg.scancmp().set_bit());
            true
        } else {
            false
        }
    }) {
        free(|cs| {
            let mut state = POWER_STATE.borrow(cs).borrow_mut();
            match *state {
                PowerState::Saving => {
                    let mut core = CORE_PERIPHERALS.borrow(cs).borrow_mut();
                    core.SCB.clear_sleeponexit();
                    adc_icmp(THRESHOLD_VOLTAGE, 0);
                    unsafe {
                        NVIC::unmask(Interrupt::LDMA);
                        NVIC::unmask(Interrupt::GPIO_EVEN);
                        NVIC::unmask(Interrupt::TIMER2);
                    }
                    *state = PowerState::Active;
                },
                PowerState::Active => {
                    adc_icmp(CUT_OUT_VOLTAGE, THRESHOLD_VOLTAGE);
                    NVIC::mask(Interrupt::LDMA);
                    NVIC::mask(Interrupt::GPIO_EVEN);
                    NVIC::mask(Interrupt::TIMER2);
                    *state = PowerState::Saving;
                    let mut core = CORE_PERIPHERALS.borrow(cs).borrow_mut();
                    core.SCB.set_sleeponexit();
                } 
            };
        });
    }
}

pub fn wait_for_energy() {
    free(|cs| {
        let mut state = POWER_STATE.borrow(cs).borrow_mut();
        *state = PowerState::Saving;
        adc_icmp(0, INIT_VOLTAGE_THRESHOLD);
        adc_cmp_ien();
        cortex_m::asm::wfi(); // wait for voltage raise
    });
}

pub fn voltage() -> i32 {
    let value = read_adc();
    value * 211 / 10000
}
