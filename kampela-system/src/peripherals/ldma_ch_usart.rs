use core::cell::Cell;
use core::ptr::addr_of;

use alloc::boxed::Box;
use cortex_m::asm::delay;
use cortex_m::interrupt::{free, Mutex};
use efm32pg23_fix::Peripherals;
use kampela_ui::display_def::*;

use crate::devices::display::Bounds;
use crate::draw::PixelBuffer;
use crate::peripherals::gpio_pins::DISP_DC_PIN;
use crate::peripherals::ldma_ch_usart_rx::CHUNK_SIZE;
use crate::{if_in_free, in_free};
use crate::peripherals::ldma::*;

pub use crate::peripherals::ldma_ch_usart_rx::RamCopy;
pub use crate::peripherals::usart::{display_select_command, display_select_data, select_display};

use super::ldma_ch_usart_rx::{ch_usart0_rx_idis, LDMAchUSART0Rx, Receivable};

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
            Some(Transmittable::RamCopy(mut a)) => {
                if a.iter_chunk() {
                    a.set_receive();
                    LDMAchUSART0Rx::set_static_cell(Some(Receivable::RamCopy(a)));
                } else {
                    ch_usart0_rx_idis();
                };
            },
            _ => ()
        };
    })
}

pub struct LDMAchUSART0(Cell<Option<Transmittable>>);

impl LDMAchUSART0 {
    pub fn init(peripherals: &Peripherals) {
        peripherals
            .ldma_s
            .if_()
            .write(|w_reg| {
                w_reg
                    .done6().clear_bit()
            });

        peripherals
            .ldmaxbar_s
            .ch6_reqsel()
            .write(|w_reg| unsafe {
                w_reg
                    .sourcesel().bits(4) // _LDMAXBAR_CH_REQSEL_SOURCESEL_USART0
                    .sigsel().bits(2) // _LDMAXBAR_CH_REQSEL_SIGSEL_USART0TXBL
            });

        peripherals
            .ldma_s
            .ch6_cfg()
            .write(|w_reg| {
                w_reg
                    .arbslots().one()
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
                        .bits(CH_USART0)
                });
        });
    }

    fn link(val: &mut Option<Transmittable>) {
        assert!(!Self::busy(), "LDMA channel is busy while tried to link");
        match val {
            None => {
                Self::unlink()
            },
            Some(t) => {
                let ChLinkData { linkaddr, loopcnt, ien } = t.link();
                in_free(|peripherals| {
                    peripherals
                        .ldma_s
                        .if_()
                        .write(|w_reg| {
                            w_reg
                                .done6().clear_bit()
                        });
                    peripherals
                        .ldma_s
                        .ch6_loop()
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
                                .chdone().bits(r_reg.chdone().bits() | 1 << CH_USART0)  
                        });
                    }
                    peripherals
                        .ldma_s
                        .chdone()
                        .write(|w_reg| {
                            w_reg
                                .chdone6().clear_bit() 
                        });
                    peripherals
                        .ldma_s
                        .ch6_link()
                        .write(|w_reg| unsafe {
                            w_reg.linkaddr().bits(linkaddr)
                        });
                    peripherals
                        .ldma_s
                        .linkload()
                        .write(|w_reg| unsafe {
                            w_reg
                                .linkload().bits(1 << CH_USART0)
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
                .bits() & (1 << CH_USART0) != 0
        })
    }

    pub fn done() -> bool {
        if_in_free(|peripherals| {
            peripherals.ldma_s.ien().read().chdone().bits() & (1 << CH_USART0) == 0 &&
            peripherals.ldma_s.chdone().read().chdone6().bit_is_set()
        })
    }

    pub fn take_static_cell() -> Option<Transmittable> {
        free(|cs| {
            LDMA_CHUSART0_TRANSMITTABLE.borrow(cs).take()
        })
    }

    pub fn take(&self) -> Option<Transmittable> {
        Self::unlink();
        self.0.take()
    }

    pub fn set_static_cell(val: Option<Transmittable>) {
        free(|cs| {
            LDMA_CHUSART0_TRANSMITTABLE.borrow(cs).set(val);
        })
    }

    pub fn set(&self, mut val: Option<Transmittable>) {
        Self::link(&mut val);
        self.0.set(val);
    }

    pub fn replace_static_cell(val: Option<Transmittable>) -> Option<Transmittable> {
        free(|cs| {
            LDMA_CHUSART0_TRANSMITTABLE.borrow(cs).replace(val)
        })
    }

    pub fn replace(&self, mut val: Option<Transmittable>) -> Option<Transmittable> {
        Self::link(&mut val);
        self.0.replace(val)
    }
}

impl Drop for LDMAchUSART0 {
    fn drop(&mut self) {
        Self::unlink();
    }
}

pub enum Transmittable {
    Display(FrameBufferLDMA),
    RamCopy(RamCopy),
    DummyTX(DummyTX),
    Array(StaticArrayLDMA)
}

impl Transmittable {
    fn link(&mut self) -> ChLinkData {
        match self {
            Transmittable::Display(a) => {
                a.link()
            },
            Transmittable::RamCopy(a) => {
                a.link()
            },
            Transmittable::DummyTX(a) => {
                a.link()
            },
            Transmittable::Array(a) => {
                a.link()
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
        let y_start_position = (SCREEN_SIZE_X - 1) as usize - bounds.2 as usize;
        let y_end_position = (SCREEN_SIZE_X - 1) as usize - bounds.3 as usize;
        let x_start_position = bounds.0 as usize;
        let x_end_position = bounds.1 as usize;
        let bounds_height = y_end_position - y_start_position + 1;
        let bounds_width = x_end_position - x_start_position + 1;

        let position = y_start_position * SCREEN_SIZE_WIDTH_ADDRESS + x_start_position;

        self.transfer_block[0].ctrl = USART_XFER_INITIAL | (bounds_width as u32 - 1) << 4;
        self.transfer_block[0].source = addr_of!(self.buffer.data[position]) as u32;

        if bounds_height < 2 {
            self.transfer_block[0].link = 0; // transfer first line only
            self.loop_cnt = 0;
            return
        }

        self.transfer_block[0].link = LINK_NEXT;
        self.transfer_block[1].ctrl = USART_XFER_LOOP | (bounds_width as u32 - 1) << 4;
        self.transfer_block[1].source = (SCREEN_SIZE_WIDTH_ADDRESS - bounds_width) as u32;

        let bounds_height_rest: u8 = (bounds_height.saturating_sub(u8::max_value() as usize + 2)).try_into().unwrap();

        if bounds_height_rest == 0 {
            self.transfer_block[1].link = LINK_SELF; // do not reload
            self.loop_cnt = (bounds_height - 2) as u8;
            return
        }
        self.transfer_block[1].link = LINK_SELF_NEXT;
        self.loop_cnt = u8::max_value();

        let next_position = position + (u8::max_value() as usize + 2) * SCREEN_SIZE_WIDTH_ADDRESS;

        self.transfer_block[3].ctrl = USART_XFER_INITIAL | (bounds_width as u32 - 1) << 4;
        self.transfer_block[3].source = addr_of!(self.buffer.data[next_position]) as u32;

        if bounds_height_rest < 2 {
            self.transfer_block[3].link = 0; // transfer only first line in block
            return
        };

        self.transfer_block[3].link = LINK_NEXT;
        self.transfer_block[2].source = (bounds_height_rest - 2) as u32;
        self.transfer_block[4].ctrl = USART_XFER_LOOP | (bounds_width as u32 - 1) << 4;
        self.transfer_block[4].source = (SCREEN_SIZE_WIDTH_ADDRESS - bounds_width) as u32;
    }

    fn link(&mut self) -> ChLinkData {
        in_free(|peripherals| {
            display_select_data(&mut peripherals.gpio_s);
        });
        ChLinkData {
            linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
            loopcnt: self.loop_cnt,
            ien: false
        }
        
    }
}

pub const USART_MATCHEN_TX: u8 = 1 << 6;
pub const USART_TX_MATCHVAL: u8 = 1 << 6;
const USART_SYNCCLEAR_TX: u8 = USART_MATCHEN_TX & !USART_TX_MATCHVAL;

pub struct DummyTX {
    _dummy_byte: Box<u8>,
    transfer_block: Box<[Descriptor; 2]>,
}
impl DummyTX {
    pub fn new(len: usize) -> Self {
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
                ctrl: USART_XFER_DUMMY | (len as u32 - 1) << 4,
                source: addr_of!(*_dummy_byte) as u32,
                dest: USART_TXDATA,
                link: 0,
            },
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
        });
        ChLinkData {
            linkaddr: addr_of!(self.transfer_block[0]) as u32 >> 2,
            loopcnt: self.loop_cnt,
            ien: false
        }
    }
}
