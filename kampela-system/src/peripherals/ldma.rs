use efm32pg23_fix::Peripherals;

use super::{ldma_ch_usart::LDMAchUSART0, ldma_ch_usart_rx::LDMAchUSART0Rx};

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

    LDMAchUSART0::init(peripherals);
    LDMAchUSART0Rx::init(peripherals);
}
