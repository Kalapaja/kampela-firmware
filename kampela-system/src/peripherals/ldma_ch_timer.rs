use core::{cell::Cell, ptr::addr_of};

use alloc::boxed::Box;
use cortex_m::interrupt::{free, Mutex};

use crate::{in_free, peripherals::ldma::*};

const XFERCNT_2047: u32 = (2048 - 1) << 4; // one less than desired 2048
const DONEIEN_TRUE: u32 = 1 << 20;
const SRCINC_NONE: u32 = 3 << 24;
const LINK_DESCRIPTORS: u32 = SIZE_HALFWORD | SRCINC_NONE | DONEIEN_TRUE | XFERCNT_2047;

pub const CH_TIM0: u8 = 7;
const TIMER0_CC0_ICF: u32 = 0x40048074;

pub const NFC_BUF_THIRD: usize = 2048;

static NFC_BUFFER_EXIST: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

pub struct NfcBuffer { // Boxed data is never moved
    pub buffer: Box<[u16; 3*NFC_BUF_THIRD]>,
    transfer_block: Box<[Descriptor; 3]>
}

impl core::ops::Deref for NfcBuffer {
    type Target = [u16; 3*NFC_BUF_THIRD];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl NfcBuffer {
    pub fn new() -> Self {
        if free(|cs| {
            NFC_BUFFER_EXIST.borrow(cs).replace(true)
        }) {
            panic!("can't be more than one instance of nfc buffer")
        }

        let buffer = Box::new([1; 3*NFC_BUF_THIRD]);
        let transfer_block = Box::new([
            Descriptor {
                ctrl: LINK_DESCRIPTORS,
                source: TIMER0_CC0_ICF,
                dest: addr_of!(buffer[0]) as u32,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: LINK_DESCRIPTORS,
                source: TIMER0_CC0_ICF,
                dest: addr_of!(buffer[NFC_BUF_THIRD]) as u32,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: LINK_DESCRIPTORS,
                source: TIMER0_CC0_ICF,
                dest: addr_of!(buffer[2*NFC_BUF_THIRD]) as u32,
                link: LINK_ANTE_PREV,
            },
        ]);
        let a = Self {
            buffer,
            transfer_block
        };
        a.init_ldma_nfc();
        a
    }

    fn init_ldma_nfc(&self) {
        in_free(|peripherals| {
            // start ldma transfer
            peripherals
                .ldma_s
                .if_()
                .write(|w_reg| {
                    w_reg
                        .done7().clear_bit()
                }
            );

            peripherals
                .ldmaxbar_s
                .ch7_reqsel()
                .write(|w_reg| unsafe {
                    w_reg
                        .sigsel().bits(0) // _LDMAXBAR_CH_REQSEL_SIGSEL_TIMER0CC0
                        .sourcesel().bits(2) // _LDMAXBAR_CH_REQSEL_SOURCESEL_TIMER0
                }
            );

            peripherals
                .ldma_s
                .ch7_loop()
                .write(|w_reg| unsafe {
                    w_reg
                        .loopcnt().bits(0)
                }
            );

            peripherals
                .ldma_s
                .ch7_cfg()
                .write(|w_reg| {
                    w_reg
                        .arbslots().one()
                        .srcincsign().positive()
                        .dstincsign().positive()
                }
            );
            
            peripherals
                .ldma_s
                .ch7_link()
                .write(|w_reg| {
                    w_reg
                        .link().clear_bit();
                    unsafe {
                        w_reg.linkaddr().bits(addr_of!(self.transfer_block[0]) as u32 >> 2)
                    }
                }
            );

            // there starts a critical section
            peripherals
                .ldma_s
                .ien()
                .modify(|r_reg, w_reg| unsafe {
                    w_reg
                        .chdone().bits(r_reg.chdone().bits() | (1 << CH_TIM0))
                }
            );

            peripherals
                .ldma_s
                .synchwen()
                .reset(); // default values, i.e. 0 for clr_off, clr_on, set_off, set_on

            peripherals
                .ldma_s
                .chdone()
                .write(|w_reg| {
                    w_reg
                        .chdone7().clear_bit()
                }
            );

            peripherals
                .ldma_s
                .linkload()
                .write(|w_reg| unsafe {
                    w_reg
                        .linkload().bits(1 << CH_TIM0)
                }
            );
        });
    }
}

impl Drop for NfcBuffer {
    fn drop(&mut self) {
        in_free(|peripherals| {
            peripherals
                .ldmaxbar_s
                .ch7_reqsel()
                .write(|w_reg| unsafe {
                    w_reg
                        .sourcesel().bits(0)
                }
            );
            peripherals
                .ldma_s
                .ien()
                .modify(|r_reg, w_reg| unsafe {
                    w_reg
                        .chdone().bits(r_reg.chdone().bits() & !(1 << CH_TIM0))
                }
            );
            peripherals
                .ldma_s
                .chdis()
                .write(|w_reg| unsafe {
                    w_reg
                        .chdis().bits(1 << CH_TIM0)
                });
        });
        free(|cs| {
            NFC_BUFFER_EXIST.borrow(cs).set(false);
        })
    }
}