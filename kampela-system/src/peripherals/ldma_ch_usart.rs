use core::cell::Cell;
use core::ptr::addr_of;

use alloc::boxed::Box;
use cortex_m::interrupt::{free, CriticalSection, Mutex};
use efm32pg23_fix::Peripherals;
use kampela_ui::display_def::*;

use crate::draw::Bounds;
use crate::draw::BoundsTrait;
use crate::draw::PixelBuffer;
use crate::in_free;
use crate::peripherals::ldma::*;

pub use crate::peripherals::ldma_ch_usart_rx::DisplayRamCopy;
pub use crate::peripherals::usart::{display_select_command, display_select_data, select_display};

use super::ldma_ch_usart_rx::{ch_usart0_rx_idis, LDMAchUSART0Rx, ReceivableUSART};

const CH_USART0: u8 = 6;
pub const USART_TXDATA: u32 = 0x4005C03C;
const CH_USART_LOOPCNT: u32 = 0x40041180;
pub const USART_CMD: u32 = 0x4005D014;

const USART_XFER: u32 = DSTINC_NONE;
pub const USART_XFER_INITIAL: u32 = DSTINC_NONE | IGNORESREQ_TRUE | BLOCKSIZE_2;
const USART_XFER_LOOP: u32 = SRCMODE_RELATIVE | DSTINC_NONE | IGNORESREQ_TRUE | DECLOOPCNT_TRUE | BLOCKSIZE_2;
pub const USART_WRI: u32 = STRUCTTYPE_WRI;
pub const USART_SYNC: u32 = STRUCTTYPE_SYNC;

const USART_XFER_DUMMY: u32 = DSTINC_NONE | SRCINC_NONE | IGNORESREQ_TRUE | BLOCKSIZE_2;

static LDMA_CHUSART0_TRANSMITTABLE: Mutex<LDMAchUSART0> = Mutex::new(LDMAchUSART0(Cell::new(None)));

pub fn ldma_display_interrupt() {
    in_free(|peripherals| {
        peripherals
        .ldma_s
        .ien()
        .modify(|r_reg, w_reg| unsafe {
            w_reg
                .chdone().bits(r_reg.chdone().bits() & !(1 << CH_USART0))  
        });
    });
    free(|cs| {
        let transmittable = LDMA_CHUSART0_TRANSMITTABLE.borrow(cs).take();

        match transmittable {
            Some(TransmittableUSART::RamCopy(mut a)) => {
                if a.iter_chunk() {
                    a.set_receive();
                    LDMAchUSART0Rx::set_static_cell(Some(ReceivableUSART::DisplayRamCopy(a)));
                } else {
                    ch_usart0_rx_idis();
                };
            },
            _ => ()
        };
    })
}

pub struct LDMAchUSART0(Cell<Option<TransmittableUSART>>);

impl LdmaCh<TransmittableUSART, CH_USART0> for LDMAchUSART0 {
    fn init(peripherals: &Peripherals) {
        peripherals
            .ldmaxbar_s
            .ch6_reqsel()
            .write(|w_reg| unsafe {
                w_reg
                    .sourcesel().bits(4) // _LDMAXBAR_CH_REQSEL_SOURCESEL_USART0
                    .sigsel().bits(2) // _LDMAXBAR_CH_REQSEL_SIGSEL_USART0TXBL
            });
    }

    fn get_static<'a>(cs: &'a CriticalSection) -> &'a Self {
        LDMA_CHUSART0_TRANSMITTABLE.borrow(cs)
    }

    fn get_cell<'a>(&'a self) -> &'a Cell<Option<TransmittableUSART>> {
        &self.0
    }
}

impl Drop for LDMAchUSART0 {
    fn drop(&mut self) {
        self.take();
    }
}

pub enum TransmittableUSART {
    Display(FrameBufferLDMA),
    RamCopy(DisplayRamCopy),
    DummyTX(DummyTX),
    Array(StaticArrayLDMA)
}

impl ChObjEnum for TransmittableUSART {
    fn link(&mut self) -> ChLinkData {
        match self {
            TransmittableUSART::Display(a) => {
                a.link()
            },
            TransmittableUSART::RamCopy(a) => {
                a.link()
            },
            TransmittableUSART::DummyTX(a) => {
                a.link()
            },
            TransmittableUSART::Array(a) => {
                a.link()
            }
        }
    }
    fn unlink(&mut self) {
        match self {
            TransmittableUSART::Display(a) => {
                a.unlink();
            },
            TransmittableUSART::RamCopy(a) => {
                a.unlink();
            },
            TransmittableUSART::DummyTX(a) => {
                a.unlink();
            },
            TransmittableUSART::Array(a) => {
                a.unlink();
            }
        }
    }
}

pub struct FrameBufferLDMA { // Boxed data is never moved
    buffer: Box<PixelBuffer>,
    transfer_block: Box<[Descriptor; 5]>,
    loop_cnt: u8,
}

impl core::ops::Deref for FrameBufferLDMA {
    type Target = PixelBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl core::ops::DerefMut for FrameBufferLDMA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl FrameBufferLDMA {
    pub fn new() -> Self {
        let buffer = Box::new(PixelBuffer::new_white());

        let transfer_block = Box::new([
            Descriptor {
                ctrl: USART_XFER_INITIAL,
                source: addr_of!(buffer.data[0]) as u32,
                dest: USART_TXDATA,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: USART_XFER_LOOP,
                source: 0,
                dest: USART_TXDATA,
                link: LINK_SELF,
            },
            Descriptor {
                ctrl: USART_WRI,
                source: 0,
                dest: CH_USART_LOOPCNT,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: USART_XFER_INITIAL,
                source: addr_of!(buffer.data[0]) as u32,
                dest: USART_TXDATA,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: USART_XFER_LOOP,
                source: 0,
                dest: USART_TXDATA,
                link: LINK_SELF,
            }
        ]);
        Self {
            buffer,
            transfer_block,
            loop_cnt: 0,
        }
    }

    pub fn set_bounds(&mut self, bounds: Bounds) {
        let position = bounds.start_bytes();

        self.transfer_block[0].ctrl = USART_XFER_INITIAL | (bounds.width_bytes() as u32 - 1) << 4;
        self.transfer_block[0].source = addr_of!(self.buffer.data[position]) as u32;

        if bounds.height() < 2 {
            self.transfer_block[0].link = 0; // transfer first line only
            self.loop_cnt = 0;
            return
        }

        self.transfer_block[0].link = LINK_NEXT;
        self.transfer_block[1].ctrl = USART_XFER_LOOP | (bounds.width_bytes() as u32 - 1) << 4;
        self.transfer_block[1].source = SCREEN_SIZE_WIDTH_ADDRESS as u32 - bounds.width_bytes() as u32 ;

        let bounds_height_rest: u8 = (bounds.height().saturating_sub(u8::max_value() as u16 + 2)).try_into().unwrap();

        if bounds_height_rest == 0 {
            self.transfer_block[1].link = LINK_SELF; // do not reload
            self.loop_cnt = (bounds.height() - 2) as u8;
            return
        }
        self.transfer_block[1].link = LINK_SELF_NEXT;
        self.loop_cnt = u8::max_value();

        let next_position = position + (u8::max_value() as usize + 2) * SCREEN_SIZE_WIDTH_ADDRESS;

        self.transfer_block[3].ctrl = USART_XFER_INITIAL | (bounds.width_bytes() as u32 - 1) << 4;
        self.transfer_block[3].source = addr_of!(self.buffer.data[next_position]) as u32;

        if bounds_height_rest < 2 {
            self.transfer_block[3].link = 0; // transfer only first line in block
            return
        };

        self.transfer_block[3].link = LINK_NEXT;
        self.transfer_block[2].source = (bounds_height_rest - 2) as u32;
        self.transfer_block[4].ctrl = USART_XFER_LOOP | (bounds.width_bytes() as u32 - 1) << 4;
        self.transfer_block[4].source = SCREEN_SIZE_WIDTH_ADDRESS as u32 - bounds.width_bytes() as u32;
    }

    fn link(&mut self) -> ChLinkData {
        in_free(|peripherals| {
            display_select_data(&mut peripherals.gpio_s);
            peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxdis().set_bit());
        });
        ChLinkData {
            linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
            loopcnt: self.loop_cnt,
            ien: false
        }
        
    }

    fn unlink(&mut self) {
        in_free(|peripherals| {
            while peripherals.usart0_s.status().read().txc().bit_is_clear() {}
        });
    }
}

pub const USART_MATCHEN_TX: u8 = 1 << 6;
pub const USART_TX_MATCHVAL: u8 = 1 << 6;
const USART_SYNCCLEAR_TX: u8 = USART_MATCHEN_TX & !USART_TX_MATCHVAL;

pub struct DummyTX {
    _dummy_byte: Box<u8>,
    transfer_block: Box<[Descriptor; 3]>,
}
impl DummyTX {
    pub fn new(len: u32) -> Self {
        assert!(len > 0, "Nothing to transmit via DMA");
        assert!(len <= 2048, "length no more than maximum xfercnt implemented");
        let _dummy_byte = Box::new(0x00);
        
        let transfer_block = Box::new([
            Descriptor {
                ctrl: USART_SYNC, // wait start rx
                source: 0,
                dest: (USART_MATCHEN_TX as u32) << 8 | USART_TX_MATCHVAL as u32,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: USART_SYNC,
                source: (USART_SYNCCLEAR_TX as u32) << 8,
                dest: 0,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: USART_XFER_DUMMY | (len - 1) << 4,
                source: addr_of!(*_dummy_byte) as u32,
                dest: USART_TXDATA,
                link: 0,
            }
        ]);

        Self {
            _dummy_byte,
            transfer_block
        }
    }

    fn link(&mut self) -> ChLinkData {
        ChLinkData {
            linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
            loopcnt: 0,
            ien: false,
        }
    }

    fn unlink(&mut self) {}
}

pub struct StaticArrayLDMA { // Boxed data is never moved
    transfer_block: Box<[Descriptor; 2]>,
    loop_cnt: u8,
}

impl StaticArrayLDMA {
    pub fn new<const LEN: usize>(array: &'static [u8; LEN]) -> Self {
        let mut transfer_block = Box::new([
            Descriptor {
                ctrl: USART_XFER_LOOP,
                source: 0,
                dest: USART_TXDATA,
                link: LINK_NEXT,
            },
            Descriptor {
                ctrl: USART_XFER_LOOP | (0x800 - 1) << 4,
                source: 0,
                dest: USART_TXDATA,
                link: LINK_SELF,
            }
        ]);

        assert!(LEN > 0, "Nothing to transmit via DMA");
        let loop_cnt = LEN.saturating_div(0x800);
        let xfercnt = (LEN as u32).saturating_sub(loop_cnt as u32 * 0x800);

        transfer_block[0].source = addr_of!(array[0]) as u32;

        if loop_cnt < 1 || xfercnt == 0  {
            transfer_block[0].link = 0; // transfer first part only or no remainder part
        } else {
            transfer_block[0].link = LINK_NEXT;
        }

        let loop_cnt = if xfercnt == 0 {
            transfer_block[0].ctrl = USART_XFER_LOOP | (0x800 - 1) << 4;
            transfer_block[0].link = LINK_SELF;
            loop_cnt.saturating_sub(1).try_into().expect("Max loop_cnt is 0xFF")
        } else {
            transfer_block[0].ctrl = USART_XFER_INITIAL | (xfercnt as u32 - 1) << 4;
            loop_cnt.try_into().expect("Max loop_cnt is 0xFF")
        };
        
        Self {
            transfer_block,
            loop_cnt
        }
    }

    fn link(&mut self) -> ChLinkData {
        in_free(|peripherals| {
            display_select_data(&mut peripherals.gpio_s);
            peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxdis().set_bit());
        });
        ChLinkData {
            linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
            loopcnt: self.loop_cnt,
            ien: false
        }
    }

    fn unlink(&mut self) {
        in_free(|peripherals| {
            while peripherals.usart0_s.status().read().txc().bit_is_clear() {}
            peripherals.usart0_s.cmd().write(|w_reg| w_reg.rxen().set_bit());
        });
    }
}
