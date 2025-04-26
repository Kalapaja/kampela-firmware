//! Power measurement unit

use crate::{if_in_free, parallel::{AsyncOperation, Threads}, peripherals::adc};


pub struct ADC {
    threads: Threads<ADCState, 1>,
    last_value: i32,
}

pub enum ADCState {
    Ready,
    Request,
}

impl Default for ADCState {
    fn default() -> Self {
        ADCState::Request
    }
}

impl ADC {
    pub fn read(&self) -> i32 {
        self.last_value * 211 / 10000
    }
}

impl AsyncOperation for ADC {
    type Init = ();
    type Input<'a> = ();
    type Output = ();

    fn new(_: ()) -> Self {
        Self{
            threads: Threads::new(ADCState::Ready),
            last_value: 0,
        }
    }

    fn advance(&mut self, _: Self::Input<'_>) {
        match self.threads.turn() {
            ADCState::Ready => {
                adc::reset_int_flags();
                adc::request_adc_measure();
                self.threads.change(ADCState::Request);
            },
            ADCState::Request => {
                if if_in_free(|peripherals| adc::read_int_flag(peripherals)) {
                    self.last_value = adc::read_adc();
                    adc::reset_int_flags();
                    adc::request_adc_measure();
                }
            },
        }
    }
}

