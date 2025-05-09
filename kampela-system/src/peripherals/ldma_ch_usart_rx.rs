use core::cell::Cell;
use core::ptr::addr_of;

use alloc::boxed::Box;
use cortex_m::interrupt::{free, CriticalSection, Mutex};
use efm32pg23_fix::Peripherals;

use crate::devices::display_transmission::{epaper_write_command, epaper_write_data};
use crate::devices::flash::{flash_cmd, flash_write_some};
use crate::devices::psram::{psram_write_read_byte, psram_write_slice, AddressPsram, PSRAM_WRITE};
use crate::draw::{Bounds, BoundsTrait};
use crate::flash_mnemonic::{WORDLIST_BASE, WORDLIST_SIZE};
use crate::flash_write_addr;
use crate::in_free;

use crate::peripherals::ldma::*;

use crate::peripherals::ldma_ch_usart::{USART_TXDATA, USART_XFER_INITIAL};
use crate::peripherals::usart::{display_select_data, select_flash};

use super::eusart::{deselect_psram, select_psram};
use super::ldma_ch_eusart::{LDMAchEUSART2, TransmittableEUSART, EUSART2_TXDATA, EUSART_XFER_INITIAL};
use super::ldma_ch_usart::*;
use super::usart::deselect_flash;

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
            Some(ReceivableUSART::DisplayRamCopy(mut a)) => {
                a.set_transmit();
                in_free(|peripherals| {
                    while peripherals.usart0_s.status().read().txc().bit_is_clear() {}
                });
                LDMAchUSART0::set_static_cell(Some(TransmittableUSART::RamCopy(a)));
                ch_usart0_rx_ien(); // mark ch as undone
            },
            Some(ReceivableUSART::FlashPSRamCopy(mut a)) => {
                in_free(|peripherals| {
                    while peripherals.usart0_s.status().read().txc().bit_is_clear() {}
                    deselect_flash(&mut peripherals.gpio_s);
                });
                a.set_transmit();
                LDMAchEUSART2::set_static_cell(Some(TransmittableEUSART::FlashPSRamCopy(a)));
                ch_usart0_rx_ien(); // mark ch as undone
            }
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

pub struct LDMAchUSART0Rx(Cell<Option<ReceivableUSART>>);

impl LdmaCh<ReceivableUSART, CH_USART0_RX> for LDMAchUSART0Rx {
    fn init(peripherals: &Peripherals) {
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

    fn get_static<'a>(cs: &'a CriticalSection) -> &'a Self {
        LDMA_CHUSART0_RECEIVABLE.borrow(cs)
    }

    fn get_cell<'a>(&'a self) -> &'a Cell<Option<ReceivableUSART>> {
        &self.0
    }
}

impl Drop for LDMAchUSART0Rx {
    fn drop(&mut self) {
        self.take();
    }
}

pub enum ReceivableUSART {
    DisplayRamCopy(DisplayRamCopy),
    FlashPSRamCopy(FlashPSRamCopy)
}

impl ChObjEnum for ReceivableUSART {
    fn link(&mut self) -> ChLinkData {
        match self {
            ReceivableUSART::DisplayRamCopy(a) => {
                a.link()
            },
            ReceivableUSART::FlashPSRamCopy(a) => {
                a.link()
            }
        }
    }
    fn unlink(&mut self) {
        match self {
            ReceivableUSART::DisplayRamCopy(a) => {
                a.unlink();
            },
            ReceivableUSART::FlashPSRamCopy(a) => {
                a.unlink();
            }
        }
    }
}

const CHUNK_SIZE: u32 = 0x200; // no more than 0x800
const USART_SYNCTRIG_TX: u8 = USART_TX_MATCHVAL;
const USART_SYNCCLEAR_TX: u8 = USART_MATCHEN_TX & !USART_TX_MATCHVAL;

enum RamCopyState {
    Receive,
    Transmit
}

pub struct DisplayRamCopy {
    buffer: Box<[u8; CHUNK_SIZE as usize]>,
    transfer_block: Box<[Descriptor; 2]>,
    left_len: u32,
    state: RamCopyState,
    current_chunk: u32
}

impl DisplayRamCopy {
    pub fn new(bounds: Bounds) -> Self {
        let buffer = Box::new([0; CHUNK_SIZE as usize]);

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
            }
        ]);

        let total_len = bounds.total_bytes();

        let mut a = Self {
            buffer,
            transfer_block,
            left_len: total_len,
            state: RamCopyState::Receive,
            current_chunk: core::cmp::min(total_len, CHUNK_SIZE)
        };
        a.set_receive();
        a
    }

    pub fn set_receive(&mut self) {
        self.transfer_block[1].ctrl = USART_XFER_RECEIVE | (self.current_chunk as u32 - 1) << 4;
        self.transfer_block[1].source = USART_RXDATA;
        self.transfer_block[1].dest = addr_of!(self.buffer[0]) as u32;

        self.state = RamCopyState::Receive;
    }

    pub fn set_transmit(&mut self) {
        self.transfer_block[1].ctrl = USART_XFER_INITIAL | (self.current_chunk as u32 - 1) << 4;
        self.transfer_block[1].source = addr_of!(self.buffer[0]) as u32;
        self.transfer_block[1].dest = USART_TXDATA;

        self.state = RamCopyState::Transmit;
    }

    pub fn iter_chunk(&mut self) -> bool {
        self.left_len = self.left_len.saturating_sub(CHUNK_SIZE);
        self.current_chunk = core::cmp::min(self.left_len, CHUNK_SIZE);
        self.left_len > 0
    }

    pub fn link(&mut self) -> ChLinkData {
        match self.state {
            RamCopyState::Receive => {
                in_free(|peripherals| {
                    peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxdis().set_bit().clearrx().set_bit());
                    epaper_write_command(peripherals, &[0x27]);
                    epaper_write_data(peripherals, &[0x0]);
                    peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxen().set_bit());
                });
                LDMAchUSART0::set_static_cell(
                    Some(
                        TransmittableUSART::DummyTX(
                            DummyTX::new(self.current_chunk)
                        )
                    )
                );
                ChLinkData {
                    linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
                    loopcnt: 0,
                    ien: true, // to switch Transmission
                }
            },
            RamCopyState::Transmit => {
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

    pub fn unlink(&mut self) {
        in_free(|peripherals| {
            while peripherals.usart0_s.status().read().txc().bit_is_clear() {}
        })
    }
}

pub const PSRAM_BLOCK_SIZE: u32 = 0x400; // should be aligned to 0x400 PSRAM block size
enum FlashPSRamCopyState {
    Receive,
    Transmit
}

pub struct FlashPSRamCopy {
    pub buffer: Box<[u8; PSRAM_BLOCK_SIZE as usize]>,
    transfer_block: Box<[Descriptor; 2]>,
    pub left_len: u32,
    state: FlashPSRamCopyState,
    current_chunk: u32,
    addr: u32,
}

impl FlashPSRamCopy {
    pub fn new() -> Self {
        let buffer = Box::new([0; PSRAM_BLOCK_SIZE as usize]);

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
        ]);

        let mut a = Self {
            buffer,
            transfer_block,
            left_len: WORDLIST_SIZE as u32,
            state: FlashPSRamCopyState::Receive,
            current_chunk: core::cmp::min(WORDLIST_SIZE as u32, PSRAM_BLOCK_SIZE),
            addr: WORDLIST_BASE
        };
        a.set_receive();
        a
    }

    pub fn set_receive(&mut self) {
        self.transfer_block[1].ctrl = USART_XFER_RECEIVE | (self.current_chunk - 1) << 4;
        self.transfer_block[1].source = USART_RXDATA;
        self.transfer_block[1].dest = addr_of!(self.buffer[0]) as u32;

        self.state = FlashPSRamCopyState::Receive;
    }

    pub fn set_transmit(&mut self) {
        self.transfer_block[1].ctrl = EUSART_XFER_INITIAL | (self.current_chunk - 1) << 4;
        self.transfer_block[1].source = addr_of!(self.buffer[0]) as u32;
        self.transfer_block[1].dest = EUSART2_TXDATA;

        self.state = FlashPSRamCopyState::Transmit;
    }

    pub fn iter_chunk(&mut self) -> bool {
        self.addr = self.addr + self.current_chunk;
        self.left_len = self.left_len.saturating_sub(PSRAM_BLOCK_SIZE);
        self.current_chunk = core::cmp::min(self.left_len, PSRAM_BLOCK_SIZE);
        self.left_len > 0
    }

    pub fn link(&mut self) -> ChLinkData {
        match self.state {
            FlashPSRamCopyState::Receive => {
                in_free(|peripherals| {
                    peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxdis().set_bit().clearrx().set_bit());
                    peripherals.usart0_s.ien().write(|w_reg| w_reg.rxof().set_bit());
                    peripherals.usart0_s.if_clr().write(|w_reg| w_reg.rxof().set_bit());
                    select_flash(&mut peripherals.gpio_s);
                    flash_cmd(peripherals, crate::devices::flash::FlashCommand::Read);
                    flash_write_addr!(peripherals, self.addr);
                    peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxen().set_bit());
                });
                LDMAchUSART0::set_static_cell(
                    Some(
                        TransmittableUSART::DummyTX(
                            DummyTX::new(self.current_chunk)
                        )
                    )
                );
                ChLinkData {
                    linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
                    loopcnt: 0,
                    ien: true, // to switch Transmission
                }
            },
            FlashPSRamCopyState::Transmit => {
                in_free(|peripherals| {
                    select_psram(&mut peripherals.gpio_s);
                    psram_write_read_byte(peripherals, PSRAM_WRITE);
                    psram_write_slice(peripherals, &AddressPsram::new(self.addr).expect("WORDLIST should fit into PSRAM").inner());
                    peripherals.eusart2_s.cmd().write(|w_reg| w_reg.rxdis().set_bit());
                });
                ChLinkData {
                    linkaddr: addr_of!(self.transfer_block[1]) as u32 >> 2,
                    loopcnt: 0,
                    ien: true, // to switch Transmission
                }
            }
        }

    }
    pub fn unlink(&mut self) {
        in_free(|peripherals| {
            while peripherals.eusart2_s.status().read().txc().bit_is_clear() {}
            deselect_psram(&mut peripherals.gpio_s);
            peripherals.eusart2_s.txdata().write(|w_reg| unsafe { w_reg.txdata().bits(0) }); // no idea why
            while peripherals.eusart2_s.status().read().txc().bit_is_clear() {}
            peripherals.eusart2_s.cmd().write(|w_reg| w_reg.rxen().set_bit());

        });
    }
}

