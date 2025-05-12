
use efm32pg23_fix::Peripherals;
use crate::{in_free, peripherals::gpio_pins::*};

/// Init timers
pub fn init_timers(peripherals: &mut Peripherals) {
    init_timer0(peripherals);
    Timer2::init(peripherals);
}

/// set up TIMER0 for NFC reading
fn init_timer0(peripherals: &mut Peripherals) {
    peripherals
        .gpio_s
        .timer0_routeen()
        .write(|w_reg| w_reg.cc0pen().set_bit());
    peripherals
        .gpio_s
        .timer0_cc0route()
        .write(|w_reg| unsafe {
            w_reg
                .port().bits(PORT_A)
                .pin().bits(NFC_PIN)
    });

    // synchronizing
    while peripherals.timer0_s.en().read().en().bit_is_set() & peripherals.timer0_s.status().read().syncbusy().bit_is_set() {}

    peripherals
        .timer0_s
        .en()
        .write(|w_reg| w_reg.en().clear_bit());

    while peripherals.timer0_s.en().read().disabling().bit_is_set() {}

    peripherals
        .timer0_s
        .cc0_cfg()
        .write(|w_reg| {
            w_reg
                .mode().inputcapture()
                .coist().clear_bit()
                .filt().disable()
                .insel().pin()
    });
    
    peripherals
        .timer0_s
        .cfg()
        .write(|w_reg| {
            w_reg
                .mode().up()
                .sync().disable()
                .osmen().clear_bit()
                .qdm().x2()
                .debugrun().run()
                .dmaclract().clear_bit()
                .clksel().prescem01grpaclk()
                .dissyncout().dis()
                .ati().clear_bit()
                .presc().div1()
                
    });

    peripherals
        .timer0_s
        .en()
        .write(|w_reg| w_reg.en().set_bit());

    peripherals
        .timer0_s
        .cc0_ctrl()
        .write(|w_reg| {
            w_reg
                .icevctrl().falling()
                .icedge().falling()
                .cufoa().none()
                .cofoa().none()
                .cmoa().none()
                .outinv().set_bit()
    });

    peripherals
        .timer0_s
        .cmd()
        .write(|w_reg| w_reg.stop().set_bit());

    peripherals
        .timer0_s
        .cnt()
        .reset();

    peripherals
        .timer0_s
        .ctrl()
        .write(|w_reg| {
            w_reg
                .risea().none()
                .falla().reloadstart()
                .x2cnt().clear_bit()
    });

 
}

macro_rules! int_timer {
    ($(#[$attr:meta] $timer_n: ident, $timer_s: tt), *) => {
        $(
            #[$attr]
            pub struct $timer_n;

            impl $timer_n {
                fn init(peripherals: &mut Peripherals) {
                    peripherals
                        .$timer_s
                        .en()
                        .write(|w_reg| w_reg.en().clear_bit());
            
                    while peripherals.$timer_s.en().read().disabling().bit_is_set() {}
            
                    peripherals
                        .$timer_s
                        .cfg()
                        .write(|w_reg| {
                            w_reg
                                .mode().up()
                                .sync().disable()
                                .osmen().set_bit()
                                .debugrun().run()
                                .clksel().prescem01grpaclk()
                                .presc().div1024()
                            });
            
                    peripherals
                        .$timer_s
                        .en()
                        .write(|w_reg| w_reg.en().set_bit());
                }
            
                pub fn reset_if_ien() {
                    in_free(|peripherals| {
                        peripherals
                            .$timer_s
                            .ien()
                            .write(|w_reg| w_reg.of().clear_bit());
                        peripherals
                            .$timer_s
                            .if_clr()
                            .write(|w_reg| w_reg.of().set_bit());
                    });
                }
                
                pub fn load(delay: u32) {
                    let timer_top = (((19_000 / 1024) * delay) as u16).saturating_sub(1);
                    in_free(|peripherals| {
                        peripherals
                            .$timer_s
                            .ien()
                            .write(|w_reg| {
                                w_reg.of().set_bit()
                            });
                        peripherals
                            .$timer_s
                            .cnt()
                            .reset();
                
                        peripherals
                            .$timer_s
                            .top()
                            .write(|w_reg| unsafe { w_reg.top().bits(timer_top) });
                
                        peripherals
                            .$timer_s
                            .cmd()
                            .write(|w_reg| w_reg.start().set_bit());
                    });
                }
            }
        )*
    }
}

int_timer!{
    /// set up TIMER2 for Touch non-blocking read
    Timer2,
    timer2_s
}