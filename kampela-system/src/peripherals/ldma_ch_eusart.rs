use core::cell::Cell;

use cortex_m::interrupt::{free, CriticalSection, Mutex};
use efm32pg23_fix::Peripherals;

use crate::in_free;
use crate::peripherals::ldma::*;

pub use crate::peripherals::ldma_ch_usart_rx::DisplayRamCopy;
pub use crate::peripherals::usart::{display_select_command, display_select_data, select_display};

use super::ldma_ch_usart_rx::{ch_usart0_rx_idis, FlashPSRamCopy, LDMAchUSART0Rx, ReceivableUSART};

const CH_EUSART2: u8 = 4;
pub const EUSART2_TXDATA: u32 = 0x400A4044;

pub const EUSART_XFER_INITIAL: u32 = DSTINC_NONE | IGNORESREQ_TRUE | BLOCKSIZE_16;

static LDMA_CHEUSART2_TRANSMITTABLE: Mutex<LDMAchEUSART2> = Mutex::new(LDMAchEUSART2(Cell::new(None)));

pub fn ldma_eusart_interrupt() {
    in_free(|peripherals| {
        peripherals
        .ldma_s
        .ien()
        .modify(|r_reg, w_reg| unsafe {
            w_reg
                .chdone().bits(r_reg.chdone().bits() & !(1 << CH_EUSART2))  
        });
    });
    free(|cs| {
        let transmittable = LDMA_CHEUSART2_TRANSMITTABLE.borrow(cs).take();

        match transmittable {
            Some(TransmittableEUSART::FlashPSRamCopy(mut a)) => {
                if a.iter_chunk() {
                    a.set_receive();
                    LDMAchUSART0Rx::set_static_cell(Some(ReceivableUSART::FlashPSRamCopy(a)));
                } else {
                    ch_usart0_rx_idis();
                };
            },
            _ => ()
        };
    })
}

pub struct LDMAchEUSART2(Cell<Option<TransmittableEUSART>>);

impl LdmaCh<TransmittableEUSART, CH_EUSART2> for LDMAchEUSART2 {
    fn init(peripherals: &Peripherals) {
        peripherals
            .ldma_s
            .if_()
            .write(|w_reg| {
                w_reg
                    .done4().clear_bit()
            });

        peripherals
            .ldmaxbar_s
            .ch4_reqsel()
            .write(|w_reg| unsafe {
                w_reg
                    .sourcesel().bits(0x12) // _LDMAXBAR_CH_REQSEL_SOURCESEL_EUSART2
                    .sigsel().bits(1) // _LDMAXBAR_CH_REQSEL_SIGSEL_EUSART2TXFL
            });

        peripherals
            .ldma_s
            .ch4_cfg()
            .write(|w_reg| {
                w_reg
                    .arbslots().one()
                    .srcincsign().positive()
                    .dstincsign().positive()
            });
    }

    fn get_static<'a>(cs: &'a CriticalSection) -> &'a Self {
        LDMA_CHEUSART2_TRANSMITTABLE.borrow(cs)
    }

    fn get_cell<'a>(&'a self) -> &'a Cell<Option<TransmittableEUSART>> {
        &self.0
    }
}

impl Drop for LDMAchEUSART2 {
    fn drop(&mut self) {
        self.take();
    }
}

pub enum TransmittableEUSART {
    FlashPSRamCopy(FlashPSRamCopy),
}

impl ChObjEnum for TransmittableEUSART {
    fn link(&mut self) -> ChLinkData {
        match self {
            TransmittableEUSART::FlashPSRamCopy(a) => {
                a.link()
            },
        }
    }
    fn unlink(&mut self) {
        match self {
            TransmittableEUSART::FlashPSRamCopy(a) => {
                a.unlink();
            },
        }
    }
}
