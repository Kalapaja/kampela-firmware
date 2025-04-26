//! Async touch panel operations
//!
//! This matches devices::touch blocking flow; TODO: replace that flow with this one
use efm32pg23_fix::Peripherals;
use cortex_m::asm::delay;

use crate::peripherals::i2c::{acknowledge_i2c_tx, acknowledge_i2c_tx_free, check_i2c_errors, check_i2c_errors_free, I2CError, mstop_i2c_wait_and_clear, mstop_i2c_wait_and_clear_free, ReadI2C};
use crate::peripherals::gpio_pins::{disable_touch_int_flag, enable_touch_int_flag, touch_res_clear};
use crate::parallel::{AsyncOperation, Timer, DELAY, Threads, WithDelay};
use crate::{in_free, if_in_free};

pub const FT6X36_REG_CHIPID: u8 = 0xA3;
pub const LEN_CHIPID: usize = 1;

pub const FT6X36_REG_NUM_TOUCHES: u8 = 0x02;
pub const LEN_NUM_TOUCHES: usize = 5;

/*
/// Blocking write function
/// TODO: replace with advance in blocking loop
pub fn ft6336_write_to(peripherals: &mut Peripherals, position: u8, data: u8) -> Result<(), I2CError> {
    // abort unexpected processes
    if peripherals
        .i2c0_s
        .state
        .read()
        .busy()
        .bit_is_set()
    {
        peripherals
            .i2c0_s
            .cmd
            .write(|w_reg| w_reg.abort().set_bit());
        delay(10000);
    }

    // clear pending commands and tx
    peripherals
        .i2c0_s
        .cmd
        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
    delay(10000);

    // clear rx buffer content
    while peripherals
        .i2c0_s
        .status
        .read()
        .rxdatav()
        .bit_is_set()
    {
        let _dummy_data = peripherals
            .i2c0_s
            .rxdata
            .read()
            .bits();
        delay(10000);
    }
    
    // clear interrupt flags
    peripherals
        .i2c0_s
        .if_
        .reset();
    
    // enable interrupts sources
    peripherals
        .i2c0_s
        .ien
        .write(|w_reg| w_reg.nack().set_bit().ack().set_bit().mstop().set_bit().rxdatav().set_bit().arblost().set_bit().buserr().set_bit());

    // i2c transfer sequence

    check_i2c_errors(peripherals)?;
    
    // send address `0x38 << 1`, for writing data
    peripherals
        .i2c0_s
        .txdata
        .write(|w_reg| w_reg.txdata().variant(0b1110000));
    delay(10000);
    peripherals
        .i2c0_s
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    // send position, single byte
    peripherals
        .i2c0_s
        .txdata
        .write(|w_reg| w_reg.txdata().variant(position));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;

    // send data to record at position, single byte
    peripherals
        .i2c0_s
        .txdata
        .write(|w_reg| w_reg.txdata().variant(data));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    peripherals
        .i2c0_s
        .cmd
        .write(|w_reg| w_reg.stop().set_bit());
    delay(10000);
    
    mstop_i2c_wait_and_clear(peripherals)?;
    
    // disable interrupts sources
    peripherals
        .i2c0_s
        .ien
        .reset();
    
    Ok(())
}


/// Blocking read function
/// TODO: replace with advance in blocking loop
pub fn ft6336_read_at<const LEN: usize>(peripherals: &mut Peripherals, position: u8) -> Result<[u8; LEN], I2CError> {
    // abort unexpected processes
    if peripherals
        .i2c0_s
        .state
        .read()
        .busy()
        .bit_is_set()
    {
        peripherals
            .i2c0_s
            .cmd
            .write(|w_reg| w_reg.abort().set_bit());
        delay(10000);
    }

    // clear pending commands and tx
    peripherals
        .i2c0_s
        .cmd
        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
    delay(10000);

    // clear rx buffer content
    while peripherals
        .i2c0_s
        .status
        .read()
        .rxdatav()
        .bit_is_set()
    {
        let _dummy_data = peripherals
            .i2c0_s
            .rxdata
            .read()
            .bits();
        delay(10000);
    }
    
    // clear interrupt flags
    peripherals
        .i2c0_s
        .if_
        .reset();
    
    // enable interrupts sources
    peripherals
        .i2c0_s
        .ien
        .write(|w_reg| w_reg.nack().set_bit().ack().set_bit().mstop().set_bit().rxdatav().set_bit().arblost().set_bit().buserr().set_bit());

    // i2c transfer sequence

    check_i2c_errors(peripherals)?;
    
    // send address `0x38 << 1`, for writing data
    peripherals
        .i2c0_s
        .txdata
        .write(|w_reg| w_reg.txdata().variant(0b1110000));
    delay(10000);
    peripherals
        .i2c0_s
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    // transfer write data, single byte
    peripherals
        .i2c0_s
        .txdata
        .write(|w_reg| w_reg.txdata().variant(position));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;

    // send address `(0x38 << 1)|1`, for reading data
    peripherals
        .i2c0_s
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    delay(10000);
    peripherals
        .i2c0_s
        .txdata
        .write(|w_reg| w_reg.txdata().variant(0b1110001));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    let mut rx_data_collected: Vec<u8> = Vec::with_capacity(LEN);
    
    for i in 0..LEN {
        rx_data_collected.push(read_i2c_rx(peripherals)?);
        if i == LEN-1 {
            peripherals
                .i2c0_s
                .cmd
                .write(|w_reg| w_reg.nack().set_bit());
            delay(10000);
            peripherals
                .i2c0_s
                .cmd
                .write(|w_reg| w_reg.stop().set_bit());
            delay(10000);
        } else {
            peripherals
                .i2c0_s
                .cmd
                .write(|w_reg| w_reg.ack().set_bit());
            delay(10000);
        }
    }
    
    mstop_i2c_wait_and_clear(peripherals)?;
    
    // disable interrupts sources
    peripherals
        .i2c0_s
        .ien
        .reset();
    
    Ok(rx_data_collected.try_into().expect("constant size, always fit"))
}
*/
pub fn clear_touch_if() {
    in_free(|peripherals| {
        peripherals
            .gpio_s
            .if_clr()
            .write(|w_reg| w_reg.extif0().set_bit());
    })
}

pub fn is_touch_int() -> bool {
    if_in_free(|peripherals| {
        peripherals
            .gpio_s
            .if_()
            .read()
            .extif0()
            .bit_is_set()
    })
}

pub fn enable_touch_int() {
    in_free(|peripherals| { //TODO: use Interupt Flag
        enable_touch_int_flag(&mut peripherals.gpio_s)
    })
}

pub fn disable_touch_int() {
    in_free(|peripherals| { //TODO: use Interupt Flag
        disable_touch_int_flag(&mut peripherals.gpio_s)
    })
}

pub fn init_touch(peripherals: &mut Peripherals) {
    touch_res_clear(&mut peripherals.gpio_s);
    delay(6000000); // datasheet: 300ms after resetting
    // abort previous operations
    if peripherals
        .i2c0_s
        .state()
        .read()
        .busy()
        .bit_is_set()
    {
        peripherals
            .i2c0_s
            .cmd()
            .write(|w_reg| w_reg.abort().set_bit());
        delay(10000);
    }
    // clear command and tx
    peripherals
        .i2c0_s
        .cmd()
        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
    delay(10000);

    // clear interrupt flags
    peripherals
        .i2c0_s
        .if_()
        .reset();

    // enable interrupts sources
    peripherals
        .i2c0_s
        .ien()
        .write(|w_reg| 
            w_reg
                .nack().set_bit()
                .ack().set_bit()
                .mstop().set_bit()
                .arblost().set_bit()
                .buserr().set_bit()
        );

    // i2c transfer sequence
    peripherals
        .i2c0_s
        .cmd()
        .write(|w_reg| w_reg.start().set_bit());
    delay(10000);

    // sending device ID
    check_i2c_errors_free(peripherals).unwrap();
    // send address `0x38 << 1`, for writing data
    peripherals
        .i2c0_s
        .txdata()
        .write(|w_reg| unsafe {w_reg.txdata().bits(0b1110000) });
    delay(10000);

    // Send address to write data
    acknowledge_i2c_tx_free(peripherals).unwrap();
    peripherals
        .i2c0_s
        .txdata()
        .write(|w_reg| unsafe { w_reg.txdata().bits(0xA4) });
    delay(10000);

    // send data
    acknowledge_i2c_tx_free(peripherals).unwrap();
    peripherals
        .i2c0_s
        .txdata()
        .write(|w_reg| unsafe { w_reg.txdata().bits(0x00) });
    delay(10000);

    // stop communication
    peripherals
        .i2c0_s
        .cmd()
        .write(|w_reg| w_reg.stop().set_bit());
    mstop_i2c_wait_and_clear_free(peripherals).unwrap();

    // cleanup
    peripherals
        .i2c0_s
        .ien()
        .reset();
}

/// Touchpad driver async state machine; value represents timer counter to counteract hardware line
/// weirdness - on count to 0 operation is supposed to be executed. Timer check does not capture
/// critical section, operation does.
pub struct Read<const LEN: usize, const POS: u8> {
    buffer: [u8; LEN],
    threads: Threads<ReadState<LEN>, 1>,
}

pub enum ReadState<const LEN: usize> {
    /// Initial idle state
    Init,
    /// Safe initial state - aborting possible previous operations
    ///
    /// command and Tx buffers should be cleaned at this point
    ClearCommand,
    /// Make sure Rx is clear and start operation by preparing to send device address
    ClearRx,
    /// Prepare sending address to read data
    PrepareAddress(Option<Timer>),
    /// Initiate address write communication by sending device ID
    AddressSendId(Option<()>),
    /// Send data to write address
    SendAddress(Option<WithDelay<()>>),
    /// Prepare reading answer
    PrepareRead(Option<Timer>),
    /// Initiate read communication by sending device ID
    ReadSendId(Option<()>),
    /// Reading
    Read(Option<ReadLoop<LEN>>),
    /// Final state, to cleanup and report result
    Aftermath,

    Error,
}

impl <const LEN: usize> Default for ReadState<LEN> {
    fn default() -> Self { ReadState::Error }
}

impl <const LEN: usize, const POS: u8> Read<LEN, POS> {

}

impl <const LEN: usize, const POS: u8> AsyncOperation for Read<LEN, POS> {
    type Init = ();
    type Input<'a> = ();
    type Output = Result<Option<Option<[u8; LEN]>>, I2CError>;

    fn new(_: ()) -> Self {
        Self {
            buffer: [0; LEN],
            threads: Threads::new(ReadState::Init), // better to calculate maximum simultaneous threads and initiate with capacity
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            ReadState::Init => {
                // abort unexpected processes
                in_free(|peripherals|
                    if peripherals
                        .i2c0_s
                        .state()
                        .read()
                        .busy()
                        .bit_is_set()
                    {
                        peripherals
                            .i2c0_s
                            .cmd()
                            .write(|w_reg| w_reg.abort().set_bit());
                    }
                );
                self.threads.change(ReadState::ClearCommand);
                Ok(Some(None))
            },
            ReadState::ClearCommand => {
                in_free(|peripherals| {
                    peripherals
                        .i2c0_s
                        .cmd()
                        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
                });
                self.threads.change(ReadState::ClearRx);
                Ok(Some(None))
            },
            ReadState::ClearRx => {
                if if_in_free(|peripherals|
                    peripherals
                        .i2c0_s
                        .status()
                        .read()
                        .rxdatav()
                        .bit_is_set()
                ) {
                    in_free(|peripherals| {
                        let _dummy_data = peripherals
                            .i2c0_s
                            .rxdata()
                            .read()
                            .bits();
                    });
                    return Ok(None)
                } else {
                    in_free(|peripherals| {
                        // clear interrupt flags
                        peripherals
                            .i2c0_s
                            .if_()
                            .reset();

                        // enable interrupts sources
                        peripherals
                            .i2c0_s
                            .ien()
                            .write(|w_reg| 
                                w_reg
                                    .nack().set_bit()
                                    .ack().set_bit()
                                    .mstop().set_bit()
                                    .arblost().set_bit()
                                    .buserr().set_bit()
                            );
                    });
                    self.threads.change(ReadState::PrepareAddress(None));
                }
                Ok(Some(None))
            },
            ReadState::PrepareAddress(state) => {
                match state {
                    None => {
                        in_free(|peripherals| {
                            peripherals
                                .i2c0_s
                                .cmd()
                                .write(|w_reg| w_reg.start().set_bit());
                        });
                        self.threads.change(ReadState::PrepareAddress(Some(Timer::new(DELAY)))); // setup time 4.7μs
                    },
                    Some(t) => {
                        if t.tick() {
                            return Ok(None)
                        }
                        self.threads.change(ReadState::AddressSendId(None));
                    }
                }
                Ok(Some(None))
            }
            ReadState::AddressSendId(state) => {
                match state {
                    None => {
                        // i2c transfer sequence
                        check_i2c_errors()?;
                        // send address `0x38 << 1`, for writing data
                        in_free(|peripherals| {
                            peripherals
                                .i2c0_s
                                .txdata()
                                .write(|w_reg| unsafe { w_reg.txdata().bits(0b1110000) });
                        });
                        self.threads.change(ReadState::AddressSendId(Some(())));
                    },
                    Some(_) => {
                        if !acknowledge_i2c_tx()? {
                            return Ok(None)
                        }
                        self.threads.change(ReadState::SendAddress(None));
                    }
                }
                Ok(None)
            },
            ReadState::SendAddress(state) => { //TODO expand this
                match state {
                    None => {
                        in_free(|peripherals| {
                            peripherals
                                .i2c0_s
                                .txdata()
                                .write(|w_reg| unsafe { w_reg.txdata().bits(POS) });
                        });
                        self.threads.change(ReadState::SendAddress(Some(WithDelay::Do(()))));
                    },
                    Some(w) => {
                        match w {
                            WithDelay::Do(_) => {
                                if !acknowledge_i2c_tx()? {
                                    return Ok(None)
                                };
                                self.threads.change(ReadState::SendAddress(Some(WithDelay::Wait(Timer::new(DELAY)))));
                            },
                            WithDelay::Wait(t) => {
                                if t.tick() {
                                    return Ok(None)
                                }
                                self.threads.change(ReadState::PrepareRead(None));
                            }
                        }
                    }
                }
                Ok(Some(None))
            },
            ReadState::PrepareRead(state) => {
                match state {
                    None => {
                        in_free(|peripherals| {
                            peripherals
                                .i2c0_s
                                .cmd()
                                .write(|w_reg| w_reg.start().set_bit());
                        });
                        self.threads.change(ReadState::PrepareRead(Some(Timer::new(DELAY)))); // setup time 4.7μs
                    },
                    Some(t) => {
                        if t.tick() {
                            return Ok(None)
                        }
                        self.threads.change(ReadState::ReadSendId(None));
                    }
                }
                Ok(Some(None))
            },
            ReadState::ReadSendId(state) => {
                match state {
                    None => {
                        // i2c transfer sequence
                        check_i2c_errors()?;
                        in_free(|peripherals| {
                            peripherals
                                .i2c0_s
                                .txdata()
                                .write(|w_reg| unsafe {w_reg.txdata().bits(0b1110001) });
                        });
                        self.threads.change(ReadState::ReadSendId(Some(())));
                    },
                    Some(_) => {
                        if !acknowledge_i2c_tx()? {
                            return Ok(None)
                        };
                        self.threads.change(ReadState::Read(None));
                    }
                }
                Ok(Some(None))
            },
            ReadState::Read(state) => {
                match state {
                    None => {
                        self.threads.change(ReadState::Read(Some(ReadLoop::<LEN>::new(()))));
                    },
                    Some(a) => {
                        match a.advance(())? {
                            Some(Some(b)) => {
                                self.buffer = b;
                                self.threads.change(ReadState::Aftermath);
                            },
                            Some(None) => {
                                return Ok(Some(None))
                            },
                            None => {
                                return Ok(None)
                            }
                        }
                    }
                }
                Ok(Some(None))
            },
            ReadState::Aftermath => {
                if !mstop_i2c_wait_and_clear()? {
                    return Ok(None)
                };
                in_free(|peripherals|
                    peripherals
                        .i2c0_s
                        .ien()
                        .reset()
                );
                Ok(Some(Some(self.buffer)))
            },
            ReadState::Error => {
                panic!("Unknown ReadState while reading touch")
            }
        }
    }
}

pub struct ReadLoop<const LEN: usize> {
    threads: Threads<ReadLoopState, 1>,
    value: [u8; LEN],
}

pub enum ReadLoopState {
    /// Read cycle
    Read(Option<(ReadI2C, usize)>),
    /// Stop reading and report result
    Aftermath,
    Error,
}

impl Default for ReadLoopState {
    fn default() -> Self { ReadLoopState::Error }
}

impl <const LEN: usize> AsyncOperation for ReadLoop<LEN> {
    type Init = ();
    type Input<'a> = ();
    type Output = Result<Option<Option<[u8; LEN]>>, I2CError>;

    fn new(_: ()) -> Self {
        Self {
            threads: Threads::new(ReadLoopState::Read(None)),
            value: [0; LEN],
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            ReadLoopState::Read(ref mut state) => {
                match state {
                    None => {
                        self.threads.change(ReadLoopState::Read(Some((ReadI2C::new(()), 0))));
                    },
                    Some((a, i)) => {
                        match a.advance(())? {
                            Some(Some(b)) => {
                                self.value[*i] = b;
                                *a = ReadI2C::new(());
                                if *i == LEN-1 {
                                    in_free(|peripherals| {
                                        peripherals
                                            .i2c0_s
                                            .cmd()
                                            .write(|w_reg| w_reg.nack().set_bit());
                                    });
                                    self.threads.change(ReadLoopState::Aftermath);
                                } else {
                                    in_free(|peripherals| {
                                        peripherals
                                            .i2c0_s
                                            .cmd()
                                            .write(|w_reg| w_reg.ack().set_bit());
                                    });
                                    *i += 1;
                                }
                            },
                            Some(None) => {
                                return Ok(Some(None))
                            },
                            None => {
                                return Ok(None)
                            }
                        }
                    }
                }
                Ok(Some(None))
            },
            ReadLoopState::Aftermath => {
                in_free(|peripherals| {
                    peripherals
                        .i2c0_s
                        .cmd()
                        .write(|w_reg| w_reg.stop().set_bit());
                });
                Ok(Some(Some(self.value)))
            },
            ReadLoopState::Error => {
                panic!("Unknown ReadLoopState while reading touch")
            }
        }
    }
}
