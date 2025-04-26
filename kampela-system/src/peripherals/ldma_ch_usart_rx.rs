use core::cell::{Cell, RefCell};
use core::ptr::addr_of;

use alloc::boxed::Box;
use cortex_m::asm::delay;
use cortex_m::interrupt::{free, Mutex};
use efm32pg23_fix::Peripherals;
use kampela_ui::display_def::*;

use crate::devices::display_transmission::{epaper_write_command, epaper_write_data};
use crate::{devices::display::Bounds, if_in_free, in_free};

use crate::peripherals::ldma::*;

use crate::peripherals::ldma_ch_usart::{USART_TXDATA, USART_WRI, USART_XFER_INITIAL};
use crate::peripherals::usart::display_select_data;

use super::ldma_ch_usart::*;
use super::usart::{deselect_display, display_select_command, write_to_usart};

const CH_USART0_RX: u8 = 5;
const USART_RXDATA: u32 = 0x4005C024;

static LDMA_CHUSART0_RECEIVABLE: Mutex<LDMAchUSART0Rx> = Mutex::new(LDMAchUSART0Rx(Cell::new(None)));
const USART_XFER_RECEIVE: u32 = SRCINC_NONE | IGNORESREQ_TRUE | BLOCKSIZE_2;

pub fn ldma_display_rx_interrupt() {
    in_free(|peripherals| {
        peripherals
            .ldma_s
            .ien()
            .modify(|r_reg, w_reg| unsafe {
                w_reg
                    .chdone().bits(r_reg.chdone().bits() & !(1 << CH_USART0_RX))  
            });
    });
    free(|cs| {
        let receivable = LDMA_CHUSART0_RECEIVABLE.borrow(cs).take();
        match receivable {
            Some(Receivable::RamCopy(mut a)) => {
                a.set_transmit();
                while !LDMAchUSART0::done() {};
                LDMAchUSART0::set_static_cell(Some(Transmittable::RamCopy(a)));
                ch_usart0_rx_ien();
            },
            _ => ()
        };
    })
}

fn ch_usart0_rx_ien() {
    in_free(|peripherals| {
        peripherals
            .ldma_s
            .ien()
            .modify(|r_reg, w_reg| unsafe {
                w_reg
                    .chdone().bits(r_reg.chdone().bits() | (1 << CH_USART0_RX))  
            });
    });
}

pub fn ch_usart0_rx_idis() {
    in_free(|peripherals| {
        peripherals
            .ldma_s
            .ien()
            .modify(|r_reg, w_reg| unsafe {
                w_reg
                    .chdone().bits(r_reg.chdone().bits() & !(1 << CH_USART0_RX))  
            });
        peripherals
            .ldma_s
            .chdone()
            .write(|w_reg| {
                w_reg
                    .chdone5().set_bit()  
            });
    });
}

pub struct LDMAchUSART0Rx(Cell<Option<Receivable>>);

impl LDMAchUSART0Rx {
    pub fn init(peripherals: &Peripherals) {
        peripherals
            .ldma_s
            .if_()
            .write(|w_reg| {
                w_reg
                    .done5().clear_bit()
            });

        peripherals
            .ldmaxbar_s
            .ch5_reqsel()
            .write(|w_reg| unsafe {
                w_reg
                    .sourcesel().bits(4) // _LDMAXBAR_CH_REQSEL_SOURCESEL_USART0
                    .sigsel().bits(0) // _LDMAXBAR_CH_REQSEL_SIGSEL_USART0RXDATAV
            });

        peripherals
            .ldma_s
            .ch5_cfg()
            .write(|w_reg| {
                w_reg
                    .arbslots().two()
                    .srcincsign().positive()
                    .dstincsign().positive()
            });
    }

    fn unlink() {
        //must be called when transmittable is dropped
        assert!(!Self::busy(), "LDMA channel is busy while tried to unlink");
        in_free(|peripherals| {
            peripherals
                .ldma_s
                .chdis()
                .write(|w_reg| unsafe {
                    w_reg
                        .chdis()
                        .bits(CH_USART0_RX)
                });
        });
    }

    fn link(val: &mut Option<Receivable>) {
        assert!(!Self::busy(), "LDMA channel is busy while tried to link");
        match val {
            None => {
                Self::unlink()
            },
            Some(t) => {
                let ChLinkData {linkaddr, loopcnt, ien} = t.link();
                in_free(|peripherals| {
                    peripherals
                        .ldma_s
                        .if_()
                        .write(|w_reg| {
                            w_reg
                                .done5().clear_bit()
                        });
                    peripherals
                        .ldma_s
                        .ch5_loop()
                        .write(|w_reg| unsafe {
                            w_reg
                                .loopcnt().bits(loopcnt)
                        });
                    if ien {
                        peripherals
                            .ldma_s
                            .ien()
                            .modify(|r_reg, w_reg| unsafe {
                                w_reg
                                    .chdone().bits(r_reg.chdone().bits() | 1 << CH_USART0_RX)  
                            });
                    }
                    peripherals
                        .ldma_s
                        .chdone()
                        .write(|w_reg| {
                            w_reg
                                .chdone5().clear_bit() 
                        });
                    peripherals
                        .ldma_s
                        .ch5_link()
                        .write(|w_reg| unsafe {
                            w_reg.linkaddr().bits(linkaddr)
                        });
                    peripherals
                        .ldma_s
                        .linkload()
                        .write(|w_reg| unsafe {
                            w_reg
                                .linkload().bits(1 << CH_USART0_RX)
                        });
                });
            }
        }
    }

    pub fn busy() -> bool {
        if_in_free(|peripherals| {
            peripherals
                .ldma_s
                .chbusy()
                .read()
                .busy()
                .bits() & (1 << CH_USART0_RX) != 0
        })
    }

    pub fn done() -> bool {
        if_in_free(|peripherals| {
            peripherals.ldma_s.ien().read().chdone().bits() & (1 << CH_USART0_RX) == 0 &&
            peripherals.ldma_s.chdone().read().chdone5().bit_is_set()
        })
    }

    pub fn take_static_cell() -> Option<Receivable> {
        free(|cs| {
            LDMA_CHUSART0_RECEIVABLE.borrow(cs).take()
        })
    }

    pub fn take(&self) -> Option<Receivable> {
        Self::unlink();
        self.0.take()
    }

    pub fn set_static_cell(val: Option<Receivable>) {
        free(|cs| {
            LDMA_CHUSART0_RECEIVABLE.borrow(cs).set(val);
        })
    }

    pub fn set(&self, mut val: Option<Receivable>) {
        Self::link(&mut val);
        self.0.set(val);
    }

    pub fn replace_static_cell(val: Option<Receivable>) -> Option<Receivable> {
        free(|cs| {
            LDMA_CHUSART0_RECEIVABLE.borrow(cs).replace(val)
        })
    }

    pub fn replace(&self, mut val: Option<Receivable>) -> Option<Receivable> {
        Self::link(&mut val);
        self.0.replace(val)
    }
}

impl Drop for LDMAchUSART0Rx {
    fn drop(&mut self) {
        Self::unlink();
    }
}

pub enum Receivable {
    RamCopy(RamCopy)
}

impl Receivable {
    fn link(&mut self) -> ChLinkData {
        match self {
            Receivable::RamCopy(a) => {
                a.link()
            },
        }
    }
}

pub const CHUNK_SIZE: usize = 0x200; // no more than 0x800
const USART_SYNCTRIG_TX: u8 = USART_TX_MATCHVAL;
const USART_SYNCCLEAR_TX: u8 = USART_MATCHEN_TX & !USART_TX_MATCHVAL;

enum RamCopyState {
    Receive(usize),
    Transmit(usize)
}

pub struct RamCopy {
    pub buffer: Box<[u8; CHUNK_SIZE]>,
    transfer_block: Box<[Descriptor; 3]>,
    pub left_len: usize,
    state: RamCopyState,
}

impl RamCopy {
    pub fn new(bounds: Bounds) -> Self {
        let buffer = Box::new([0; CHUNK_SIZE]);

        let transfer_block = Box::new([
            Descriptor { // start tx
                ctrl: USART_SYNC | STRUCTREQ_TRUE,
                source: USART_SYNCTRIG_TX as u32,
                dest: 0,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: USART_XFER_RECEIVE,
                source: USART_RXDATA,
                dest: addr_of!(buffer[0]) as u32,
                link: 0,
            },
            Descriptor {
                ctrl: USART_SYNC | STRUCTREQ_TRUE,
                source: (USART_SYNCCLEAR_TX as u32) << 8,
                dest: 0,
                link: 0,
            },
        ]);

        let y_start_position = (SCREEN_SIZE_X - 1) as usize - bounds.2 as usize;
        let y_end_position = (SCREEN_SIZE_X - 1) as usize - bounds.3 as usize;
        let x_start_position = bounds.0 as usize;
        let x_end_position = bounds.1 as usize;
        let bounds_height = y_end_position - y_start_position + 1;
        let bounds_width = x_end_position - x_start_position + 1;

        let total_len = bounds_height * bounds_width;

        let mut a = Self {
            buffer,
            transfer_block,
            left_len: total_len,
            state: RamCopyState::Receive(0)
        };
        a.set_receive();
        a
    }

    pub fn set_receive(&mut self) {
        let current_chunk = core::cmp::min(self.left_len, CHUNK_SIZE);

        self.transfer_block[1].ctrl = USART_XFER_RECEIVE | (current_chunk as u32 - 1) << 4;
        self.transfer_block[1].source = USART_RXDATA;
        self.transfer_block[1].dest = addr_of!(self.buffer[0]) as u32;
        self.transfer_block[1].link = 0;

        self.state = RamCopyState::Receive(current_chunk);
    }

    pub fn set_transmit(&mut self) {
        let current_chunk = core::cmp::min(self.left_len, CHUNK_SIZE);

        self.transfer_block[1].ctrl = USART_XFER_INITIAL | (current_chunk as u32 - 1) << 4;
        self.transfer_block[1].source = addr_of!(self.buffer[0]) as u32;
        self.transfer_block[1].dest = USART_TXDATA;
        self.transfer_block[1].link = 0;

        in_free(|peripherals| {
            peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxdis().set_bit());
        });

        self.state = RamCopyState::Transmit(current_chunk);
    }

    pub fn iter_chunk(&mut self) -> bool {
        self.left_len = self.left_len.saturating_sub(CHUNK_SIZE);
        self.left_len > 0
    }

    pub fn link(&mut self) -> ChLinkData {
        match self.state {
            RamCopyState::Receive(len) => {
                in_free(|peripherals| {
                    peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxdis().set_bit().clearrx().set_bit());
                    peripherals.usart0_s.ien().write(|w_reg| w_reg.rxof().set_bit());
                    peripherals.usart0_s.if_clr().write(|w_reg| w_reg.rxof().set_bit());
                    epaper_write_command(peripherals, &[0x27]);
                    epaper_write_data(peripherals, &[0x0]);
                    peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxen().set_bit());
                });
                LDMAchUSART0::set_static_cell(
                    Some(
                        Transmittable::DummyTX(
                            DummyTX::new(len)
                        )
                    )
                );
                ChLinkData {
                    linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
                    loopcnt: 0,
                    ien: true, // to switch Transmission
                }
            },
            RamCopyState::Transmit(_) => {
                in_free(|peripherals| {
                    epaper_write_command(peripherals, &[0x26]);
                    display_select_data(&mut peripherals.gpio_s);
                });
                ChLinkData {
                    linkaddr: addr_of!(self.transfer_block[1]) as u32 >> 2,
                    loopcnt: 0,
                    ien: true, // to switch Transmission
                }
            }
        }

    }
}

