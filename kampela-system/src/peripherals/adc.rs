//! Initializations for ADC module

use efm32pg23_fix::Peripherals;
use crate::in_free;

/// read value from ADC
pub fn read_adc() -> i32 {
    let mut value = 0;
    in_free(|peripherals|
        value = peripherals.iadc0_s.scandata().read().data().bits() & 0x00FFFFFF
    );
    (if value & 0x00800000 == 0 {
        value
    } else {
        value | 0xFF000000
    }) as i32
}

/// Initialize ADC
///
/// assumes that CMU clock is enabled and does not check it
pub fn init_adc(peripherals: &mut Peripherals) {
    // IADC_reset()
    reset_adc(peripherals);

    //CMU clockselectset
    peripherals
        .cmu_s
        .iadcclkctrl()
        .write(|w_reg| w_reg.clksel().fsrco());

    disable_adc(peripherals);

    // actually init
    peripherals
        .iadc0_s
        .ctrl()
        .write(|w_reg| {
            w_reg
                .adcclksuspend0().prswudis()
                .adcclksuspend1().prswudis()
                .dbghalt().normal()
                .warmupmode().normal()
                .hsclkrate().div1();
            unsafe {
                w_reg.timebase().bits(94)
            }
        });
    peripherals
        .iadc0_s
        .timer()
        .write(|w_reg| unsafe { w_reg.timer().bits(950) });

    peripherals
        .iadc0_s
        .cmpthr()
        .reset();

    cfg0_set(peripherals);
    
    cfg1_set(peripherals);

    init_adc_scan_reader(peripherals);

    enable_adc(peripherals);

    // set gpio
    peripherals
        .gpio_s
        .abusalloc()
        .write(|w_reg| w_reg.aeven0().adc0());

    //request_adc_measure(peripherals);

    // TODO! remove this in prod
    //
    // This allows debugger to stay connected while Kampela sleeps in EM2 and waits for power to
    // replenish
    peripherals
        .emu_s
        .ctrl()
        .write(|w_reg| w_reg.em2dbgen().set_bit());

    peripherals
        .iadc0_s
        .cmd()
        .write(|w_reg| w_reg.scanstart().set_bit().timeren().set_bit());
}

/// Calibration data for ADC defived from factory values read from memory
struct CalibrationData {
    pub offset_truncated: u32,
    pub ui_gain_sign: bool,
    pub ui_gain_value: u16,
}

impl CalibrationData {
    /// Read calibration data and convert it into ADC input.
    ///
    /// This is very hardcoded block, with only variablde `div` that differs between cfg0 and cfg1
    /// in reference code for this particular project. Could be done nicer, but we have no time for
    /// that now.
    pub fn new (peripherals: &mut Peripherals, div: i16) -> CalibrationData {
        // calibration data
        //
        // things are complicated...
        let ui_gain = peripherals
            .devinfo
            .iadc0gain0()
            .read()
            .gaincana1()
            .bits();
        let offset1 = peripherals
            .devinfo
            .iadc0normaloffsetcal1()
            .read()
            .offsetana3norm()
            .bits() as i16; // C reference did this
        let offset0 = peripherals
            .devinfo
            .iadc0normaloffsetcal0()
            .read()
            .bits() as i16; // C reference did this

        // do some floating point
        let offset = offset1 / div + offset0;
        let offset_f = offset as f32 + 87505.45;
        let offset_f = (ui_gain as f32 / 5416.198) * offset_f - 524288.0;
        let offset = offset_f as i32;

        // mess with bytes now
        //
        // signed 
        let ui_gain_sign = ui_gain & 0x8000 == 0x8000;
        let ui_gain_value = ui_gain & 0x1FFF;
        let offset_truncated = match offset {
            i32::MIN..=-0x20001 => -0x20000,
            -0x20000..=0x1FFFF => offset,
            0x20000..=i32::MAX => 0x1FFFF,
        } as u32;

        CalibrationData {
            offset_truncated,
            ui_gain_sign,
            ui_gain_value,
        }
    }
}

/// Set up cfg0 for ADC.
fn cfg0_set(peripherals: &mut Peripherals) {
    peripherals
        .iadc0_s
        .cfg0()
        .write(|w_reg| {
            w_reg
                .adcmode().normal()
                .osrhs().hispd32()
                .analoggain().anagain0p5()
                .refsel().vbgr()
                .digavg().avg16()
                .twoscompl().auto()
        });
   
    let calibrations = CalibrationData::new(peripherals, 16);

    peripherals
        .iadc0_s
        .scale0()
        .write(|w_reg| {
            unsafe {
                w_reg
                    .offset().bits(calibrations.offset_truncated)
                    .gain13lsb().bits(calibrations.ui_gain_value);
            }
            let prefab = w_reg.gain3msb();
            if calibrations.ui_gain_sign {
                prefab.gain100()
            } else {
                prefab.gain011()
            }
        });
    peripherals
        .iadc0_s
        .sched0()
        .write(|w_reg| unsafe { w_reg.prescale().bits(1) });
}

/// Set up cfg1 for ADC. Not sure it is even used.
fn cfg1_set(peripherals: &mut Peripherals) {
    peripherals
        .iadc0_s
        .cfg1()
        .reset();

    let calibrations = CalibrationData::new(peripherals, 4);

    peripherals
        .iadc0_s
        .scale1()
        .write(|w_reg| {
            unsafe {
                w_reg
                    .offset().bits(calibrations.offset_truncated)
                    .gain13lsb().bits(calibrations.ui_gain_value);
            }
            let prefab = w_reg.gain3msb();
            if calibrations.ui_gain_sign {
                prefab.gain100()
            } else {
                prefab.gain011()
            }
        });
    peripherals
        .iadc0_s
        .sched1()
        .write(|w_reg| unsafe { w_reg.prescale().bits(1)});
}

/// Initialize single ADC read config
fn init_adc_scan_reader(peripherals: &mut Peripherals) {
    enable_adc(peripherals);
    peripherals
        .iadc0_s
        .maskreq()
        .write(|w_reg| unsafe {
            w_reg.maskreq().bits(1 << 0)
        });

    disable_adc(peripherals);
    
    peripherals
        .iadc0_s
        .scanfifocfg()
        .write(|w_reg| {
            w_reg
                .alignment().right20()
                .showid().clear_bit()
                .dvl().valid1()
                .dmawufifoscan().disabled()
        });
    
    peripherals
        .iadc0_s
        .trigger()
        .modify(|_, w_reg| {
            w_reg
                .scantrigsel().timer()
                .scantrigaction().continuous()
        });

    // measure between GND and PA0
    peripherals
        .iadc0_s
        .scan0()
        .write(|w_reg| {
            w_reg
                .portneg().gnd()
                .portpos().porta()
                .cfg().config0()
                .cmp().clear_bit();
            unsafe {
                w_reg.pinpos().bits(0)
            }
        });

}

/// Enable ADC
fn enable_adc(peripherals: &mut Peripherals) {
        peripherals
        .iadc0_s
        .en()
        .write(|w_reg| w_reg.en().enable());
}

/// Disable ADC
fn disable_adc(peripherals: &mut Peripherals) {
    while 
        peripherals
            .iadc0_s
            .status()
            .read()
            .syncbusy()
            .bit_is_set()
    {}
    peripherals
        .iadc0_s
        .en()
        .write(|w_reg| w_reg.en().disable());

    while
        peripherals
            .iadc0_s
            .en()
            .read()
            .disabling()
            .bit_is_set()
    {}
}

/// reset IADC to settings similar to those after HW reset
fn reset_adc(peripherals: &mut Peripherals) {
    enable_adc(peripherals);
    peripherals
        .iadc0_s
        .cmd()
        .write(|w_reg| {
            w_reg
                .singlestop().set_bit()
                .scanstop().set_bit()
                .timerdis().set_bit()
        });
    while
        peripherals.iadc0_s.status().read().singlequeuepending().bit_is_set() |
        peripherals.iadc0_s.status().read().scanqueuepending().bit_is_set() |
        peripherals.iadc0_s.status().read().converting().bit_is_set() |
        peripherals.iadc0_s.status().read().timeractive().bit_is_set() 
    {}
    peripherals
        .iadc0_s
        .maskreq()
        .reset();
    peripherals
        .iadc0_s
        .single()
        .reset();
    while
        peripherals.iadc0_s.status().read().singlewritepending().bit_is_set() |
        peripherals.iadc0_s.status().read().maskreqwritepending().bit_is_set()
        {}
    while
        peripherals.iadc0_s.status().read().singlefifodv().bit_is_set() |
        peripherals.iadc0_s.singlefifostat().read().fiforeadcnt().ne(&0)
    {
        let _dummy_data = peripherals
            .iadc0_s
            .singlefifodata()
            .read()
            .data();
    }
    while
        peripherals.iadc0_s.status().read().scanfifodv().bit_is_set() |
        peripherals.iadc0_s.scanfifostat().read().fiforeadcnt().ne(&0)
    {
        let _dummy_data = peripherals
            .iadc0_s
            .scanfifodata()
            .read()
            .data();
    }
    let _dummy_data = peripherals
        .iadc0_s
        .singledata()
        .read()
        .data();
    let _dummy_data = peripherals
        .iadc0_s
        .scandata()
        .read()
        .data();

    disable_adc(peripherals);

    peripherals
        .iadc0_s
        .ctrl()
        .reset();
    peripherals
        .iadc0_s
        .timer()
        .reset();
    peripherals
        .iadc0_s
        .trigger()
        .reset();
    peripherals
        .iadc0_s
        .cmpthr()
        .reset();
    peripherals
        .iadc0_s
        .singlefifocfg()
        .reset();
    peripherals
        .iadc0_s
        .scanfifocfg()
        .reset();
    peripherals
        .iadc0_s
        .cfg0()
        .reset(); 
    peripherals
        .iadc0_s
        .scale0()
        .reset();
    peripherals
        .iadc0_s
        .sched0()
        .reset();
    peripherals
        .iadc0_s
        .cfg1()
        .reset();
    peripherals
        .iadc0_s
        .scale1()
        .reset();
    peripherals
        .iadc0_s
        .sched1()
        .reset();

    peripherals
        .iadc0_s
        .scan0()
        .reset();
    peripherals
        .iadc0_s
        .scan1()
        .reset();
    peripherals
        .iadc0_s
        .scan2()
        .reset();
    peripherals
        .iadc0_s
        .scan3()
        .reset();
    peripherals
        .iadc0_s
        .scan4()
        .reset();
    peripherals
        .iadc0_s
        .scan5()
        .reset();
    peripherals
        .iadc0_s
        .scan6()
        .reset();
    peripherals
        .iadc0_s
        .scan7()
        .reset();
    peripherals
        .iadc0_s
        .scan8()
        .reset();
    peripherals
        .iadc0_s
        .scan9()
        .reset();
    peripherals
        .iadc0_s
        .scan10()
        .reset();
    peripherals
        .iadc0_s
        .scan11()
        .reset();
    peripherals
        .iadc0_s
        .scan12()
        .reset();
    peripherals
        .iadc0_s
        .scan13()
        .reset();
    peripherals
        .iadc0_s
        .scan14()
        .reset();
    peripherals
        .iadc0_s
        .scan15()
        .reset();

    peripherals
        .iadc0_s
        .if_()
        .reset();
    peripherals
        .iadc0_s
        .ien()
        .reset();
}


