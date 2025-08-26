//! EUSART interface

use efm32pg23_fix::{GpioS, Peripherals};
use crate::peripherals::gpio_pins::*;

pub const BAUDRATE_EUSART: u32 = 10_000_000;

/// Enable psram channel
pub fn select_psram(gpio: &mut GpioS) {
    psram_chip_select_clear(gpio);
}

/// Disable psram channel
pub fn deselect_psram(gpio: &mut GpioS) {
    psram_chip_select_set(gpio);
}

/// setting up EUSART2, for PSRAM
///
/// why gpio setup is before init? does the order matter at all?
pub fn init_eusart(peripherals: &mut Peripherals) {
    // PSRAM MOSI
    peripherals
        .gpio_s
        .eusart2_txroute()
        .write(|w_reg| unsafe {
            w_reg
                .port().bits(PORT_C)
                .pin().bits(PSRAM_MOSI_PIN)
    });
    // PSRAM MISO
    peripherals
        .gpio_s
        .eusart2_rxroute()
        .write(|w_reg| unsafe {
            w_reg
                .port().bits(PORT_C)
                .pin().bits(PSRAM_MISO_PIN)
    });
    // PSRAM SCK
    peripherals
        .gpio_s
        .eusart2_sclkroute()
        .write(|w_reg| unsafe {
            w_reg
                .port().bits(PORT_C)
                .pin().bits(PSRAM_SCK_PIN)
    });
    // route enable
    peripherals
        .gpio_s
        .eusart2_routeen()
        .write(|w_reg| {
            w_reg
                .txpen().set_bit()
                .rxpen().set_bit()
                .sclkpen().set_bit()
    });

    // EUSART2 init
    if peripherals
        .eusart2_s
        .en()
        .read()
        .bits()
        .ne(&0)
    {
        while peripherals.eusart2_s.syncbusy().read().bits().ne(&0) {}
    }
    
    // reset EUSART
    eusart_reset(peripherals);

    // calculate clkdiv
    let clkdiv: u8 = (19_000_000/BAUDRATE_EUSART - 1).try_into().expect("BAURATE_EUSART is expected to not exceed and be comparable to reference frequency");
    
    // configure
    peripherals
        .eusart2_s
        .cfg2()
        .write(|w_reg| {
            w_reg
                .master().master()
                .clkpol().idlelow()
                .clkpha().sampleleading()
                .csinv().al()
                .autotx().clear_bit()
                .autocs().set_bit()
                .clkprsen().clear_bit()
                .forceload().set_bit();
            unsafe {
                w_reg.sdiv().bits(clkdiv)
            }
        });
    peripherals
        .eusart2_s
        .cfg1()
        .write(|w_reg|
            w_reg
                .txfiw().sixteenframes()
                .rxfiw().oneframe()
        );
    peripherals
        .eusart2_s
        .cfg0()
        .write(|w_reg|
            w_reg
                .sync().sync()
                .loopbk().disable()
                .rxinv().disable()
                .txinv().disable()
                .msbf().enable()
        );
    peripherals
        .eusart2_s
        .timingcfg()
        .write(|w_reg| {
            w_reg
                .cssetup().zero()
                .cshold().zero()
                .ics().zero();
            unsafe {
                w_reg.setupwindow().bits(4)
            }
            });
    peripherals
        .eusart2_s
        .framecfg()
        .write(|w_reg|
            w_reg
                .databits().eight()
        );
    peripherals
        .eusart2_s
        .dtxdatcfg()
        .write(|w_reg| unsafe {
            w_reg.dtxdat().bits(0)
        });

    eusart_enable(peripherals);

    while peripherals.eusart2_s.status().read().rxidle().bit_is_clear()
        | peripherals.eusart2_s.status().read().txidle().bit_is_clear()  {}

    // remember to reset connected ram device here later, right after setup
}

fn eusart_disable(peripherals: &mut Peripherals) {
    if peripherals
        .eusart2_s
        .en()
        .read()
        .en()
        .bit_is_set() 
    {
        if peripherals.eusart2_s.cfg0().read().sync().bit_is_clear() | peripherals.eusart2_s.cfg2().read().master().bit_is_set() {
            // disable TX and RX
            peripherals.eusart2_s.cmd().write(|w_reg| w_reg.rxdis().set_bit().txdis().set_bit());

            // wait for TXDIS and RXDIS to pass
            while peripherals.eusart2_s.syncbusy().read().rxdis().bit_is_set() | peripherals.eusart2_s.syncbusy().read().txdis().bit_is_set() {}

            // wait for TX and RX enable status to go low
            while peripherals.eusart2_s.status().read().rxens().bit_is_set() | peripherals.eusart2_s.status().read().txens().bit_is_set() {}
        }
        
        peripherals
            .eusart2_s
            .en()
            .write(|w_reg| w_reg.en().clear_bit());
        
        // wait for disabling to clear
        while peripherals.eusart2_s.en().read().disabling().bit_is_set() {}
    }
}

fn eusart_enable(peripherals: &mut Peripherals) {
    peripherals
        .eusart2_s
        .en()
        .write(|w_reg| w_reg.en().set_bit());

    while peripherals.eusart2_s.syncbusy().read().bits().ne(&0) {}

    peripherals
        .eusart2_s
        .cmd()
        .write(|w_reg| {
            w_reg
                .rxen().set_bit()
                .rxdis().clear_bit()
                .txen().set_bit()
                .txdis().clear_bit()
//                .rxblockdis().set_bit() // added
//                .rxblocken().clear_bit() // added
    });

    while peripherals.eusart2_s.syncbusy().read().rxen().bit_is_set()
        | peripherals.eusart2_s.syncbusy().read().rxdis().bit_is_set()
        | peripherals.eusart2_s.syncbusy().read().txen().bit_is_set()
        | peripherals.eusart2_s.syncbusy().read().txdis().bit_is_set()
//        | peripherals.eusart2_s.syncbusy.read().rxblockdis().bit_is_set() // added
//        | peripherals.eusart2_s.syncbusy.read().rxblocken().bit_is_set() // added
    {}

    while peripherals.eusart2_s.status().read().rxens().bit_is_clear()
        | peripherals.eusart2_s.status().read().txens().bit_is_clear()
//        | peripherals.eusart2_s.status.read().rxblock().bit_is_set() // added
    {}
}

fn eusart_reset(peripherals: &mut Peripherals) {
    eusart_disable(peripherals);

    for _i in 0..4 {
        peripherals.eusart2_s.cfg2().write(|w_reg| w_reg.clkpha().set_bit());
        peripherals.eusart2_s.cfg2().write(|w_reg| w_reg.clkpha().clear_bit());
    }

    peripherals.eusart2_s.cfg2().reset();
    peripherals.eusart2_s.cfg1().reset();
    peripherals.eusart2_s.cfg0().reset();
    peripherals.eusart2_s.framecfg().reset();
    peripherals.eusart2_s.dtxdatcfg().reset();
    peripherals.eusart2_s.timingcfg().reset();
    peripherals.eusart2_s.irhfcfg().reset();
    peripherals.eusart2_s.startframecfg().reset();
    peripherals.eusart2_s.sigframecfg().reset();
    peripherals.eusart2_s.trigctrl().reset();
    peripherals.eusart2_s.ien().reset();
    peripherals.eusart2_s.if_().reset();
    peripherals.eusart2_s.clkdiv().reset();
}


