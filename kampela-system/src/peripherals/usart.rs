//! all low level usart operations

use efm32pg23_fix::{interrupt, GpioS, Peripherals};
use crate::{in_free, peripherals::{self, gpio_pins::*}};

pub const BAUDRATE_USART: u32 = 2_500_000;

/// Select display channel
pub fn select_display(gpio: &mut GpioS) {
    display_chip_select_clear(gpio);
}

/// Deselect display channel
pub fn deselect_display(gpio: &mut GpioS) {
    display_chip_select_set(gpio);
}

/// Select flash channel
pub fn select_flash(gpio: &mut GpioS) {
    flash_chip_select_clear(gpio);
}

/// Deselect flash channel
pub fn deselect_flash(gpio: &mut GpioS) {
    flash_chip_select_set(gpio);
}


/// Indicate that command is sent
pub fn display_select_command(gpio: &mut GpioS) {
    spi_data_command_clear(gpio);
}

/// Indicate that data is sent
pub fn display_select_data(gpio: &mut GpioS) {
    spi_data_command_set(gpio);
}

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
pub fn spi_is_busy(gpio: &mut GpioS) -> bool {
    let portb_din_bits = gpio.portb_din().read().din().bits();
    portb_din_bits & (1 << SPI_BUSY_PIN) == (1 << SPI_BUSY_PIN)
}

/// Initialize USART0, for EPD (display)
///
/// Assumes that clocks are enabled
pub fn init_usart(peripherals: &mut Peripherals) {
    peripherals
        .usart0_s
        .en()
        .write(|w_reg| {
            w_reg
                .en().set_bit()
    });
    peripherals
        .usart0_s
        .ctrl()
        .write(|w_reg| {
            w_reg
                .sync().enable()
                .clkpol().idlelow()
                .msbf().enable()
                .autotx().clear_bit()
    });
    peripherals
        .usart0_s
        .frame()
        .write(|w_reg| {
            w_reg
                .databits().eight()
                .stopbits().one()
                .parity().none()
    });

    let clkdiv = ((19_000_000 << 4)/(BAUDRATE_USART)).saturating_sub(32); // ((19_000_000 - 1)/(2*BAUDRATE_USART)) << 8 preventing overflow

    peripherals
        .usart0_s
        .clkdiv()
        .write(|w_reg| unsafe {
            w_reg
                .div().bits(clkdiv)
    });
    peripherals
        .usart0_s
        .cmd()
        .write(|w_reg| {
            w_reg
                .masteren().set_bit()
                .txen().set_bit()
                .rxen().set_bit()
    });
    // display MOSI
    peripherals
        .gpio_s
        .usart0_txroute()
        .write(|w_reg| unsafe {
            w_reg
                .port().bits(PORT_C)
                .pin().bits(E_MOSI_PIN)
    });
    // display MISO
    peripherals
        .gpio_s
        .usart0_rxroute()
        .write(|w_reg| unsafe {
            w_reg
                .port().bits(PORT_C)
                .pin().bits(E_MISO_PIN)
    });
    // display SCK
    peripherals
        .gpio_s
        .usart0_clkroute()
        .write(|w_reg| unsafe {
            w_reg
                .port().bits(PORT_C)
                .pin().bits(E_SCK_PIN)
    });
    // route enable
    peripherals
        .gpio_s
        .usart0_routeen()
        .write(|w_reg| {
            w_reg
                .txpen().set_bit()
                .rxpen().set_bit()
                .clkpen().set_bit()
    });

}

/// Write `u8` data to usart.
///
/// At this point USART must be already clocked from elsewhere.
pub fn write_to_usart(peripherals: &mut Peripherals, data: u8) -> u8 {
    while peripherals.usart0_s.status().read().txbl().bit_is_clear() {}

    peripherals
        .usart0_s
        .txdata()
        .write(|w_reg| unsafe { w_reg.txdata().bits(data) });

    while peripherals.usart0_s.status().read().txc().bit_is_clear() {}

    peripherals.usart0_s.rxdata().read().rxdata().bits()
}

