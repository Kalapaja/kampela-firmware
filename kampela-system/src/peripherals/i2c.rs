
use efm32pg23_fix::Peripherals;
use crate::peripherals::gpio_pins::*;
use cortex_m::asm::delay;
use crate::{if_in_free, in_free, FreeError};
use crate::parallel::{AsyncOperation, Threads};


#[derive(Debug)]
pub enum I2CError {
    ArbitrationLost,
    BusError,
    TransferNack,
    /// Peripheral mutex could not be borrowed
    PeripheralsLocked,
    /// Errors probably caused by skip in the state sequence
    SequenceError,
}

impl From<FreeError> for I2CError {
    fn from(_item: FreeError) -> Self {
        Self::PeripheralsLocked
    }
}

pub fn init_i2c(peripherals: &mut Peripherals) {
    peripherals
        .gpio_s
        .i2c0_routeen()
        .write(|w_reg| w_reg.sclpen().set_bit().sdapen().set_bit());
    peripherals
        .gpio_s
        .i2c0_sdaroute()
        .write(|w_reg| unsafe { w_reg.port().bits(PORT_A).pin().bits(SDA_PIN) });
    peripherals
        .gpio_s
        .i2c0_sclroute()
        .write(|w_reg| unsafe { w_reg.port().bits(PORT_A).pin().bits(SCL_PIN) });
    
    peripherals
        .i2c0_s
        .ien()
        .reset();
    peripherals
        .i2c0_s
        .if_()
        .reset();
    peripherals
        .i2c0_s
        .ctrl()
        .write(|w_reg| w_reg.slave().disable().clhr().standard());
    peripherals
        .i2c0_s
        .clkdiv()
        .write(|w_reg| unsafe { w_reg.div().bits(12) }); // divider calculated as 10, set to 12 for debug
    peripherals
        .i2c0_s
        .en()
        .write(|w_reg| w_reg.en().enable());
    peripherals
        .i2c0_s
        .ctrl()
        .write(|w_reg| w_reg.corerst().enable());
    delay(10000);
    peripherals
        .i2c0_s
        .ctrl()
        .write(|w_reg| w_reg.corerst().disable());
    delay(100000);
}

/// I2C bus reader for our touchpad, not generalized until someone needs it
pub struct ReadI2C {
    threads: Threads<ReadI2CState, 1>,
    value: Option<u8>,
}

pub enum ReadI2CState {
    /// Read data register
    Read,
    /// Clean state registers and output data
    OutputData
}

impl Default for ReadI2CState {
    fn default() -> Self { ReadI2CState::OutputData }
}

impl AsyncOperation for ReadI2C {
    type Init = ();
    type Input<'a> = ();
    type Output = Result<Option<Option<u8>>, I2CError>;

    fn new(_: ()) -> Self {
        Self {
            threads: Threads::new(ReadI2CState::Read),
            value: None,
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            ReadI2CState::Read => {
                check_i2c_errors()?;
                if if_in_free(|peripherals|
                    peripherals
                        .i2c0_s
                        .status()
                        .read()
                        .rxdatav()
                        .bit_is_set()
                ) {
                    in_free(|peripherals| 
                        self.value = Some(
                            peripherals
                                .i2c0_s
                                .rxdata()
                                .read()
                                .rxdata()
                                .bits()
                        )
                    );
                    self.threads.change(ReadI2CState::OutputData);
                    Ok(Some(None))
                } else {
                    Ok(None)
                }
            },
            ReadI2CState::OutputData => {
                if let Some(out) = self.value {
                    Ok(Some(Some(out)))
                } else {
                    Err(I2CError::SequenceError)
                }
            }
        }
    }
}

/*
pub fn read_i2c_sync() -> Result<u8, I2CError> {
    let mut reader = ReadI2C::new();
    loop {
        if let Some(out) = reader.advance(())? {
            return Ok(out)
        }
    }
}
*/

pub fn check_i2c_errors() -> Result<(), I2CError> {
    let mut out = Ok(());
    in_free(|peripherals| {
        out = check_i2c_errors_free(peripherals)
    });
    out
}

pub fn acknowledge_i2c_tx() -> Result<bool, I2CError> {
    check_i2c_errors()?;

    if if_in_free(|peripherals|
        peripherals
            .i2c0_s
            .if_()
            .read()
            .ack()
            .bit_is_clear()
    ) {
        check_i2c_errors()?;

        if if_in_free(|peripherals|
            peripherals
                .i2c0_s
                .if_()
                .read()
                .nack()
                .bit_is_set()
         ) {
            in_free(|peripherals| {
                // clear interrupt flag
                peripherals
                    .i2c0_s
                    .if_()
                    .write(|w_reg| w_reg.nack().clear_bit());
                // stop
                peripherals
                    .i2c0_s
                    .cmd()
                    .write(|w_reg| w_reg.stop().set_bit());
            });

            delay(100000);
            return Err(I2CError::TransferNack)
        }

        Ok(false)
    } else {
        in_free(|peripherals| {
            // clear interrupt flag
            peripherals
                .i2c0_s
                .if_()
                .write(|w_reg| w_reg.ack().clear_bit());
        });
        Ok(true)
    }
}

pub fn mstop_i2c_wait_and_clear() -> Result<bool, I2CError> {
    check_i2c_errors()?;
    if if_in_free(|peripherals|
        peripherals
            .i2c0_s
            .if_()
            .read()
            .mstop()
            .bit_is_clear()
    ) {
        check_i2c_errors()?;
        Ok(false)
    } else {
        in_free(|peripherals| {
            peripherals
            .i2c0_s
            .if_()
            .write(|w_reg| w_reg.mstop().clear_bit());
        });
        Ok(true)
    }
}

pub fn check_i2c_errors_free(peripherals: &mut Peripherals) -> Result<(), I2CError> {
    let if_read = peripherals
        .i2c0_s
        .if_()
        .read();
    if if_read.arblost().bit_is_set() {return Err(I2CError::ArbitrationLost)}
    if if_read.buserr().bit_is_set() {return Err(I2CError::BusError)}
    Ok(())
}

pub fn acknowledge_i2c_tx_free(peripherals: &mut Peripherals) -> Result<(), I2CError> {
    check_i2c_errors_free(peripherals)?;
    while peripherals
        .i2c0_s
        .if_()
        .read()
        .ack()
        .bit_is_clear()
    {
        check_i2c_errors_free(peripherals)?;

        if peripherals
            .i2c0_s
            .if_()
            .read()
            .nack()
            .bit_is_set()
        {
            // clear interrupt flag
            peripherals
                .i2c0_s
                .if_()
                .write(|w_reg| w_reg.nack().clear_bit());
            // stop
            peripherals
                .i2c0_s
                .cmd()
                .write(|w_reg| w_reg.stop().set_bit());
            delay(100000);
            return Err(I2CError::TransferNack)
        }
    }
    // clear interrupt flag
    peripherals
        .i2c0_s
        .if_()
        .write(|w_reg| w_reg.ack().clear_bit());

    Ok(())
}

pub fn mstop_i2c_wait_and_clear_free(peripherals: &mut Peripherals) -> Result<(), I2CError> {
    check_i2c_errors_free(peripherals)?;
    while peripherals
        .i2c0_s
        .if_()
        .read()
        .mstop()
        .bit_is_clear()
    {
        check_i2c_errors_free(peripherals)?;
    }
    peripherals
        .i2c0_s
        .if_()
        .write(|w_reg| w_reg.mstop().clear_bit());
    Ok(())
}

