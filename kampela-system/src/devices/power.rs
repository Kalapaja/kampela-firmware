//! Power measurement unit
use core::cell::RefCell;

use cortex_m::{interrupt::Mutex};
use efm32pg23_fix::{interrupt, Interrupt, NVIC, SCB};

use crate::{devices::touch::init_touch, free, if_in_free, in_free, peripherals::{adc::{adc_cmp_ien, adc_icmp, read_adc}, gpio_pins::{display_res_clear, display_res_set, pow_clear, touch_res_set}}, CORE_PERIPHERALS};

const THRESHOLD_VOLTAGE: u16 = 4000;
pub const INIT_VOLTAGE_THRESHOLD: u16 = 8000;
const CUT_OUT_VOLTAGE: u16 = 2400;

static POWER_STATE: Mutex<RefCell<PowerState>> = Mutex::new(RefCell::new(PowerState::Active));

enum PowerState {
    Saving,
    Active,
    Reset,
}

#[interrupt]
fn IADC() {
    free(|cs| {
        let mut core = CORE_PERIPHERALS.borrow(cs).borrow_mut();
        core.SCB.set_sleeponexit();
        if if_in_free(|peripherals| {
            if peripherals.iadc0_s.if_().read().scancmp().bit_is_set() {
                peripherals.iadc0_s.if_clr().write(|w_reg| w_reg.scancmp().set_bit());
                true
            } else {
                false
            }
        }) {
            let mut state = POWER_STATE.borrow(cs).borrow_mut();
            if voltage() < (THRESHOLD_VOLTAGE + CUT_OUT_VOLTAGE) as i32 / 2 {
                *state = PowerState::Reset;
                in_free(|peripherals| {
                    pow_clear(&mut peripherals.gpio_s);
                });
                core.SCB.set_sleepdeep();
                adc_icmp(INIT_VOLTAGE_THRESHOLD, 0);
                return
            };
            match *state {
                PowerState::Saving => {
                    core.SCB.clear_sleeponexit();
                    *state = PowerState::Active;
                    init_active_state();
                },
                PowerState::Active => {
                    *state = PowerState::Saving;
                    init_saving_state(THRESHOLD_VOLTAGE);
                },
                PowerState::Reset => {
                    SCB::sys_reset();
                }
            };
        }
    });
}

pub fn wait_for_energy(level: u16) {
    free(|cs| {
        let mut state = POWER_STATE.borrow(cs).borrow_mut();
        *state = PowerState::Saving;
    });
    init_saving_state(level);
    cortex_m::asm::wfi(); // wait for voltage raise
}

pub fn wait_for_energy_init() {
    if voltage() >= THRESHOLD_VOLTAGE as i32 {
        init_active_state();
        adc_cmp_ien();
        return
    }
    adc_icmp(INIT_VOLTAGE_THRESHOLD, 0);
    in_free(|peripherals| {
        touch_res_set(&mut peripherals.gpio_s);
        display_res_set(&mut peripherals.gpio_s);
        pow_clear(&mut peripherals.gpio_s);
    });
    adc_cmp_ien();
    free(|cs| {
        let mut state = POWER_STATE.borrow(cs).borrow_mut();
        *state = PowerState::Reset; // to make sure everything initialized correctly
        let mut core = CORE_PERIPHERALS.borrow(cs).borrow_mut();
        core.SCB.set_sleepdeep();
    });
    cortex_m::asm::wfi(); // wait for voltage raise
}

fn init_saving_state(level: u16) {
    adc_icmp(level, CUT_OUT_VOLTAGE);
    NVIC::mask(Interrupt::LDMA);
    NVIC::mask(Interrupt::GPIO_EVEN);
    NVIC::mask(Interrupt::TIMER2);
    in_free(|peripherals| {
        touch_res_set(&mut peripherals.gpio_s);
        display_res_set(&mut peripherals.gpio_s);
    });
}

fn init_active_state() {
    adc_icmp(0, THRESHOLD_VOLTAGE);
    unsafe {
        NVIC::unmask(Interrupt::LDMA);
        NVIC::unmask(Interrupt::GPIO_EVEN);
        NVIC::unmask(Interrupt::TIMER2);
    }
    in_free(|peripherals| {
        init_touch(peripherals);
        display_res_clear(&mut peripherals.gpio_s);
    });
}


pub fn voltage() -> i32 {
    let value = read_adc();
    value * 211 / 10000
}
