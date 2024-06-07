use efm32pg23_fix::{GPIO_S, Peripherals};
use cortex_m::asm::delay;

use crate::peripherals::usart::*;
use crate::peripherals::gpio_pins::{display_res_clear, display_res_set};
use crate::{FreeError, if_in_free, in_free};
use crate::parallel::Operation;
use kampela_display_common::display_def::*;

pub const BUFSIZE: usize = 5808;
const X_ADDRESS_WIDTH: usize = (SCREEN_SIZE_Y / 8) as usize;

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
pub fn display_is_busy() -> Result<bool, FreeError> {
    if_in_free(|peripherals| spi_is_busy(&mut peripherals.GPIO_S))
}

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
///
/// Blocking variant to be called from critical section (init, panic)
pub fn display_is_busy_cs(peripherals: &mut Peripherals) -> bool {
    spi_is_busy(&mut peripherals.GPIO_S)
}

/// Send EPD to low power state; should be performed when screen is not drawing at all times to
/// extend component life
pub fn epaper_deep_sleep(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x10]); // from manual, enter deep sleep
    epaper_write_data(peripherals, &[0x03]); // Deep sleep mode 2, cannot retain RAM data
    delay(100); // why delay, from where the number?
}

/// EPD init, also should be performed to wake screen from sleep
///
/// used within critical section
pub fn epaper_hw_init_cs(peripherals: &mut Peripherals) {
    epaper_reset(&mut peripherals.GPIO_S);
    while display_is_busy_cs(peripherals) {}
    epaper_write_command(peripherals, &[0x12]);
    delay(10000);
    while display_is_busy_cs(peripherals) {}
}

/// Reset EPD, should be performed in many situations
///
/// for critical section
///
/// Why these specific numbers for delays?
pub fn epaper_reset(gpio: &mut GPIO_S) {
    delay(1000);
    display_res_clear(gpio);
    delay(5000);
    display_res_set(gpio);
    delay(10000);
    display_res_clear(gpio);
    delay(5000);
    deselect_display(gpio); // this is not the default state, should not be here
    delay(5000);
}

/// Send command to EPD
///
/// for critical section
pub fn epaper_write_command(peripherals: &mut Peripherals, command_set: &[u8]) {
    // CS clear corresponds to selected chip, see epaper docs

    deselect_display(&mut peripherals.GPIO_S);
    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
    
    display_select_command(&mut peripherals.GPIO_S);
    for command in command_set.iter() {
        write_to_usart(peripherals, *command);
    }
    deselect_display(&mut peripherals.GPIO_S);
}

/// Send data to EPD
///
/// for critical section
pub fn epaper_write_data(peripherals: &mut Peripherals, data_set: &[u8]) {
    deselect_display(&mut peripherals.GPIO_S);
    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start

    display_select_data(&mut peripherals.GPIO_S);
    for data in data_set.iter() {
        write_to_usart(peripherals, *data);
    }
    deselect_display(&mut peripherals.GPIO_S);
    //    display_data_command_clear(peripherals);
}

/// Send command `C` to EPD
pub struct EPDCommand<const C: u8>{
    state: EPDByteState,
    timer: usize,
}

pub enum EPDByteState {
    /// State where command is actually sent
    Init,
    /// Receive something to keep protocol running and close connection
    Aftermath,
}

impl <const C: u8> Operation for EPDCommand<C> {
    type Init = ();
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = EPDByteState;

    fn new(_: ()) -> Self {
        Self {
            state: EPDByteState::Init,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDByteState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }
    
    fn advance(&mut self, _: ()) -> bool {
        match self.state {
            EPDByteState::Init => {
                in_free(|peripherals| {
                    deselect_display(&mut peripherals.GPIO_S);
                    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
                    display_select_command(&mut peripherals.GPIO_S);
                });
                if if_in_free(|peripherals|
                    peripherals.USART0_S.status.read().txbl().bit_is_clear()
                ) == Ok(false) {
                    in_free(|peripherals|
                        peripherals
                            .USART0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(C))
                            );
                    self.change(EPDByteState::Aftermath);
                }
                false
            },
            EPDByteState::Aftermath => {
                if if_in_free(|peripherals|
                    peripherals
                        .USART0_S
                        .status
                        .read()
                        .txc()
                        .bit_is_clear()
                ) == Ok(false) { 
                    in_free(|peripherals| {
                        peripherals
                            .USART0_S
                            .rxdata
                            .read()
                            .rxdata()
                            .bits();
                        deselect_display(&mut peripherals.GPIO_S);
                    });
                    true
                } else {
                    false
                }
            },
        }
    }
}



/// Send data byte `B` to EPD
pub struct EPDDataB<const B: u8>{
    state: EPDByteState,
    timer: usize,
}

impl <const B: u8> Operation for EPDDataB<B> {
    type Init = ();
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = EPDByteState;

    fn new(_: ()) -> Self {
        Self {
            state: EPDByteState::Init,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDByteState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, _: ()) -> bool {
        match self.state {
            EPDByteState::Init => {
                in_free(|peripherals| {
                    deselect_display(&mut peripherals.GPIO_S);
                    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
                    display_select_data(&mut peripherals.GPIO_S);
                });
                if if_in_free(|peripherals|
                    peripherals.USART0_S.status.read().txbl().bit_is_clear()
                ) == Ok(false) {
                    in_free(|peripherals|
                        peripherals
                            .USART0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(B))
                            );
                    self.change(EPDByteState::Aftermath);
                }
                false
            },
            EPDByteState::Aftermath => {
                if if_in_free(|peripherals|
                    peripherals
                        .USART0_S
                        .status
                        .read()
                        .txc()
                        .bit_is_clear()
                ) == Ok(false) { 
                    in_free(|peripherals| {
                        peripherals
                            .USART0_S
                            .rxdata
                            .read()
                            .rxdata()
                            .bits();
                        deselect_display(&mut peripherals.GPIO_S);
                    });
                    true
                } else {
                    false
                }
            },
        }
    }
}

/// Send data byte `B` to EPD
pub struct EPDData<const LEN: usize>{
    state: EPDDataState,
    position: usize,
    timer: usize,
}

pub enum EPDDataState {
    /// State where command is actually sent
    Init,
    /// Receive something to keep protocol running and close connection
    Aftermath,
}

impl <const LEN: usize> Operation for EPDData<LEN> {
    type Init = ();
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = EPDDataState;

    fn new(_: ()) -> Self {
        Self {
            state: EPDDataState::Init,
            position: 0,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDDataState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state {
            EPDDataState::Init => {
                in_free(|peripherals| {
                    deselect_display(&mut peripherals.GPIO_S);
                    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
                    display_select_data(&mut peripherals.GPIO_S);
                });
                if if_in_free(|peripherals|
                    peripherals.USART0_S.status.read().txbl().bit_is_clear()
                ) == Ok(false) {
                    in_free(|peripherals|
                        peripherals
                            .USART0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(data[self.position]))
                            );
                    if self.position < LEN-1 {
                        self.position += 1;
                    } else {
                        self.change(EPDDataState::Aftermath);
                    }
                }
                false
            },
            EPDDataState::Aftermath => {
                if if_in_free(|peripherals|
                    peripherals
                        .USART0_S
                        .status
                        .read()
                        .txc()
                        .bit_is_clear()
                ) == Ok(false) { 
                    in_free(|peripherals| {
                        peripherals
                            .USART0_S
                            .rxdata
                            .read()
                            .rxdata()
                            .bits();
                        deselect_display(&mut peripherals.GPIO_S);
                    });
                    true
                } else {
                    false
                }
            },
        }
    }
}

pub struct EPDDataPart<const LEN: usize>{
    state: EPDDataState,
    position: usize,
    x_start_position: usize,
    x_end_position: usize,
    y_end_position: usize,
    timer: usize,
}



impl <const LEN: usize> Operation for EPDDataPart<LEN> {
    type Init = (u8, u8, u16, u16);
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = EPDDataState;

    fn new(addresses: Self::Init) -> Self {
        let x_start_position = addresses.0 as usize;
        let y_start_position = (SCREEN_SIZE_X - 1) as usize - addresses.2 as usize; //Y coordinates inversed for some reason
        Self {
            state: EPDDataState::Init,
            position: y_start_position * X_ADDRESS_WIDTH as usize + x_start_position,
            x_start_position,
            x_end_position: addresses.1 as usize,
            y_end_position: (SCREEN_SIZE_X - 1) as usize - addresses.3 as usize,
            timer: 0,
        }
    }

    fn wind(&mut self, state: EPDDataState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state {
            EPDDataState::Init => {
                in_free(|peripherals| {
                    deselect_display(&mut peripherals.GPIO_S);
                    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
                    display_select_data(&mut peripherals.GPIO_S);
                });
                if if_in_free(|peripherals|
                    peripherals.USART0_S.status.read().txbl().bit_is_clear()
                ) == Ok(false) {
                    in_free(|peripherals|
                        peripherals
                            .USART0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(data[self.position]))
                    );
                    
                    if self.position < X_ADDRESS_WIDTH * self.y_end_position + self.x_end_position {
                        let y_position = self.position / X_ADDRESS_WIDTH;
                        let x_position = self.position - y_position * X_ADDRESS_WIDTH;
                        
                        if x_position >= self.x_end_position {
                            self.position = (y_position + 1) * X_ADDRESS_WIDTH + self.x_start_position;
                        } else {
                            self.position += 1;
                        }
                    } else {
                        self.change(EPDDataState::Aftermath);
                    }
                }
                false
            },
            EPDDataState::Aftermath => {
                if if_in_free(|peripherals|
                    peripherals
                        .USART0_S
                        .status
                        .read()
                        .txc()
                        .bit_is_clear()
                ) == Ok(false) { 
                    in_free(|peripherals| {
                        peripherals
                            .USART0_S
                            .rxdata
                            .read()
                            .rxdata()
                            .bits();
                        deselect_display(&mut peripherals.GPIO_S);
                    });
                    true
                } else {
                    false
                }
            },
        }
    }
}