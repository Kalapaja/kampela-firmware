//! Power measurement unit
use crate::peripherals::adc::read_adc;

pub fn voltage() -> i32 {
    let value = read_adc();
    value * 211 / 10000
}
