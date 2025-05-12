use core::cell::Cell;

use cortex_m::interrupt::CriticalSection;
use efm32pg23_fix::{interrupt, Peripherals};

use cortex_m::interrupt::free;

use crate::{if_in_free, in_free};

use super::{ldma_ch_eusart::{ldma_eusart_interrupt, LDMAchEUSART2}, ldma_ch_timer::{ldma_nfc_interrupt, LDMAchTimer0}, ldma_ch_usart::{ldma_display_interrupt, LDMAchUSART0}, ldma_ch_usart_rx::{ldma_display_rx_interrupt, LDMAchUSART0Rx}};

pub const LINK_TRUE: u32 = 1 << 1;
pub const LINKMODE_RELATIVE: u32 = 1;

pub const LINKADDR_NEXT: u32 = size_of::<Descriptor>() as u32; // >> 2 for words; << 2 for link_addr 2:31
pub const LINK_NEXT: u32 = LINKADDR_NEXT | LINK_TRUE | LINKMODE_RELATIVE;

pub const LINKADDR_PREV: u32 = (u32::MIN.wrapping_sub(LINKADDR_NEXT >> 2)).overflowing_shl(2).0;
pub const LINK_PREV: u32 = LINKADDR_PREV | LINK_TRUE | LINKMODE_RELATIVE;

pub const LINKADDR_ANTE_PREV: u32 = (u32::MIN.wrapping_sub(2 * LINKADDR_NEXT >> 2)).overflowing_shl(2).0;
pub const LINK_ANTE_PREV: u32 = LINKADDR_ANTE_PREV | LINK_TRUE | LINKMODE_RELATIVE;

pub const LINK_SELF: u32 = LINKMODE_RELATIVE;
pub const LINK_SELF_NEXT: u32 = LINK_TRUE | LINKMODE_RELATIVE;

pub const STRUCTTYPE_SYNC: u32 = 1;
pub const STRUCTTYPE_WRI: u32 = 2;
pub const STRUCTREQ_TRUE: u32 = 1 << 3;
pub const BLOCKSIZE_2: u32 = 1 << 16;
pub const BLOCKSIZE_3: u32 = 2 << 16;
pub const BLOCKSIZE_16: u32 = 7 << 16;
pub const DONEIEN: u32 = 1 << 20;
pub const REQMODE_ALL: u32 = 1 << 21;
pub const DECLOOPCNT_TRUE: u32 = 1 << 22;
pub const IGNORESREQ_TRUE: u32 = 1 << 23;
pub const SRCINC_NONE: u32 = 3 << 24;
pub const SIZE_HALFWORD: u32 = 1 << 26;
pub const DSTINC_NONE: u32 = 3 << 28;
pub const SRCMODE_RELATIVE: u32 = 1 << 30;

pub struct ChLinkData {
    pub linkaddr: u32,
    pub loopcnt: u8,
    pub ien: bool,
}


#[interrupt]
fn LDMA() {
    if if_in_free(|peripherals| {
        peripherals.ldma_s.if_().read().done7().bit_is_set()
    }) {
        in_free(|peripherals| {
            peripherals.ldma_s.if_clr().write(|w_reg| {
                w_reg.done7().set_bit()
            });
        });
        ldma_nfc_interrupt();
    }
    if if_in_free(|peripherals| {
        peripherals.ldma_s.if_().read().done6().bit_is_set()
    }) {
        in_free(|peripherals| {
            peripherals.ldma_s.if_clr().write(|w_reg| {
                w_reg.done7().set_bit()
            });
        });
        ldma_display_interrupt();
    }
    if if_in_free(|peripherals| {
        peripherals.ldma_s.if_().read().done5().bit_is_set()
    }) {
        in_free(|peripherals| {
            peripherals.ldma_s.if_clr().write(|w_reg| {
                w_reg.done5().set_bit()
            });
        });
        ldma_display_rx_interrupt();
    }
    if if_in_free(|peripherals| {
        peripherals.ldma_s.if_().read().done4().bit_is_set()
    }) {
        in_free(|peripherals| {
            peripherals.ldma_s.if_clr().write(|w_reg| {
                w_reg.done4().set_bit()
            });
        });
        ldma_eusart_interrupt();
    }
    if if_in_free(|peripherals| {
        peripherals.ldma_s.if_().read().error().bit_is_set() 
    }) {
        in_free(|peripherals| {
            panic!(
                "error on ldma interrupt, ldma_status: chnum {:}, fifolevel: {:}, cherror: {:}, chgrant: {:}, anyreq: {:}, anybusy: {:}",
                peripherals.ldma_s.status().read().chnum().bits(),
                peripherals.ldma_s.status().read().fifolevel().bits(),
                peripherals.ldma_s.status().read().cherror().bits(),
                peripherals.ldma_s.status().read().chgrant().bits(),
                peripherals.ldma_s.status().read().anyreq().bit_is_set(),
                peripherals.ldma_s.status().read().anybusy().bit_is_set(),
            )
        });
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Descriptor {
    pub ctrl: u32,
    pub source: u32,
    pub dest: u32,
    pub link: u32,
}

pub fn init_ldma(peripherals: &mut Peripherals) {
    // set up ldma
    peripherals
        .ldma_s
        .en()
        .write(|w_reg| {
            w_reg
                .en().set_bit()
    });

    peripherals
        .ldma_s
        .ctrl()
        .write(|w_reg| unsafe {
            w_reg
                .numfixed().bits(0)
    });

    peripherals
        .ldma_s
        .synchwen()
        .write(|w_reg| unsafe {
            w_reg
                .syncseten().bits(0)
                .syncclren().bits(0)
    });

    peripherals
        .ldma_s
        .chdis()
        .write(|w_reg| unsafe {
            w_reg
                .chdis().bits(0xFF)
    });

    peripherals
        .ldma_s
        .dbghalt()
        .write(|w_reg| unsafe {
            w_reg
                .dbghalt().bits(0)
        });

    peripherals
        .ldma_s
        .reqdis()
        .write(|w_reg| unsafe {
            w_reg
                .reqdis().bits(0)
        });

    peripherals
        .ldma_s
        .ien()
        .write(|w_reg| {
            w_reg
                .error().set_bit()
        });

    peripherals
        .ldma_s
        .if_()
        .reset();

    LDMAchTimer0::init(peripherals);
    LDMAchUSART0::init(peripherals);
    LDMAchUSART0Rx::init(peripherals);
    LDMAchEUSART2::init(peripherals);
}

macro_rules! chx_set_loopcnt {
    ( $( $CH: ident, $periph: tt, $loopcnt: expr ),* ) => {
        match CH {
            0u8 => $($periph.ldma_s.ch0_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            1u8 => $($periph.ldma_s.ch1_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            2u8 => $($periph.ldma_s.ch2_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            3u8 => $($periph.ldma_s.ch3_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            4u8 => $($periph.ldma_s.ch4_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            5u8 => $($periph.ldma_s.ch5_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            6u8 => $($periph.ldma_s.ch6_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            7u8 => $($periph.ldma_s.ch7_loop().write(|w_reg| unsafe {w_reg.loopcnt().bits($loopcnt)}))*,
            8u8..=u8::MAX => panic!("No such ldma channel"),
        }
    };
}

macro_rules! chx_set_linkaddr {
    ( $( $CH: ident, $periph: tt, $linkaddr: expr ),* ) => {
        match CH {
            0u8 => $($periph.ldma_s.ch0_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            1u8 => $($periph.ldma_s.ch1_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            2u8 => $($periph.ldma_s.ch2_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            3u8 => $($periph.ldma_s.ch3_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            4u8 => $($periph.ldma_s.ch4_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            5u8 => $($periph.ldma_s.ch5_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            6u8 => $($periph.ldma_s.ch6_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            7u8 => $($periph.ldma_s.ch7_link().write(|w_reg| unsafe {w_reg.linkaddr().bits($linkaddr)}))*,
            8u8..=u8::MAX => panic!("No such ldma channel"),
        }
    };
}

pub trait ChObjEnum {
    fn link(&mut self) -> ChLinkData;
    fn unlink(&mut self);
}

pub trait LdmaCh<T: ChObjEnum, const CH: u8> {
    fn init(peripherals: &Peripherals);

    fn unlink(val: &mut Option<T>) {
        val.as_mut().map(|a| a.unlink());
        //must be called when transmittable is dropped
        assert!(!Self::busy(), "LDMA channel is busy while tried to unlink");
        in_free(|peripherals| {
            peripherals
                .ldma_s
                .chdis()
                .write(|w_reg| unsafe {
                    w_reg
                        .chdis()
                        .bits(CH)
                });
        });
    }

    fn link(val: &mut Option<T>) {
        assert!(!Self::busy(), "LDMA channel is busy while tried to link");
        match val {
            None => {},
            Some(t) => {
                let ChLinkData { linkaddr, loopcnt, ien } = t.link();
                in_free(|peripherals| {
                    peripherals
                        .ldma_s
                        .if_clr()
                        .write(|w_reg| unsafe{
                            w_reg.bits(1 << CH)
                        });
                    chx_set_loopcnt!(CH, peripherals, loopcnt);
                    chx_set_linkaddr!(CH, peripherals, linkaddr);
                    if ien {
                        peripherals
                        .ldma_s
                        .ien()
                        .modify(|r_reg, w_reg| unsafe {
                            w_reg
                                .chdone().bits(r_reg.chdone().bits() | 1 << CH)  
                        });
                    }
                    peripherals
                        .ldma_s
                        .chdone()
                        .modify(|r_reg, w_reg| unsafe {
                            w_reg.bits(r_reg.bits() & !(1 << CH))
                        });
                    peripherals
                        .ldma_s
                        .linkload()
                        .write(|w_reg| unsafe {
                            w_reg
                                .linkload().bits(1 << CH)
                        });
                });
            }
        }
    }

    fn busy() -> bool {
        if_in_free(|peripherals| {
            peripherals
                .ldma_s
                .chbusy()
                .read()
                .busy()
                .bits() & (1 << CH) != 0
        })
    }

    fn done() -> bool {
        if_in_free(|peripherals| {
            peripherals.ldma_s.ien().read().chdone().bits() & (1 << CH) == 0 &&
            peripherals.ldma_s.chdone().read().bits() & (1 << CH) != 0
        })
    }

    fn get_static<'a>(cs: &'a CriticalSection) -> &'a Self;

    fn take_static_cell() -> Option<T> {
        let mut a = None;
        free(|cs| {
            a = Self::get_static(cs).take()
        });
        a
    }

    fn set_static_cell(val: Option<T>) {
        free(|cs| {
            Self::get_static(cs).set(val)
        });
    }

    fn replace_static_cell(val: Option<T>) -> Option<T> {
        let mut a = None;
        free(|cs| {
            a = Self::get_static(cs).replace(val)
        });
        a
    }

    fn get_cell<'a>(&'a self) -> &'a Cell<Option<T>>;

    fn take(&self) -> Option<T> {
        let mut a = self.get_cell().take();
        Self::unlink(&mut a);
        a
    }

    fn set(&self, val: Option<T>) {
        self.replace(val);
    }

    fn replace(&self, val: Option<T>) -> Option<T> {
        let mut a = self.get_cell().replace(val);
        Self::unlink(&mut a);
        let mut val = self.get_cell().replace(a);
        Self::link(&mut val);
        self.get_cell().replace(val)
    }
}
