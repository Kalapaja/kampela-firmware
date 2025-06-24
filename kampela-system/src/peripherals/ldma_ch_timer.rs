use core::{cell::Cell, ptr::addr_of};

use alloc::boxed::Box;
use cortex_m::interrupt::{free, CriticalSection, Mutex};
use efm32pg23_fix::{Interrupt, NVIC};

use crate::{in_free, peripherals::ldma::*, CORE_PERIPHERALS};

const XFERCNT_2047: u32 = (2048 - 1) << 4; // one less than desired 2048
const DONEIEN_TRUE: u32 = 1 << 20;
const SRCINC_NONE: u32 = 3 << 24;
const LINK_DESCRIPTORS: u32 = SIZE_HALFWORD | SRCINC_NONE | XFERCNT_2047;

pub const CH_TIM0: u8 = 7;
const TIMER0_CC0_ICF: u32 = 0x40048074;

pub const NFC_BUF_THIRD: usize = 2048;

static LDMA_TIMER0_RECEIVABLE: Mutex<LDMAchTimer0> = Mutex::new(LDMAchTimer0(Cell::new(None)));
static LDMA_TIMER0_DONE: Mutex<Cell<Option<NfcReceive>>> = Mutex::new(Cell::new(None));
static LDMA_TIMER0_NEXT: Mutex<Cell<Option<NfcReceive>>> = Mutex::new(Cell::new(None));

pub fn ldma_nfc_interrupt() {
    in_free(|peripherals| {
        peripherals
            .ldma_s
            .ien()
            .modify(|r_reg, w_reg| unsafe {
                w_reg
                    .chdone().bits(r_reg.chdone().bits() & !(1 << CH_TIM0))  
            });
    });
    free(|cs| {
        ldma_nfc_switch_buffer(cs);
    });
}

fn ldma_nfc_switch_buffer(cs: &CriticalSection) {
    if let Some(done) = LDMA_TIMER0_DONE.borrow(cs).take() {
        LDMA_TIMER0_DONE.borrow(cs).set(Some(done))
    } else if let Some(next) = LDMA_TIMER0_NEXT.borrow(cs).take() {
        if let Some(done) = LDMA_TIMER0_RECEIVABLE.borrow(cs).replace(Some(ReceivableTIMER::NfcReceive(next))) {
            LDMA_TIMER0_DONE.borrow(cs).set(match done { ReceivableTIMER::NfcReceive(d) => Some(d) });
        } else {
            unreachable!("LDMA Timer0 channel buffer should always set in place while receiving")
        }
        //trigger software interrupt for nfc collection
        NVIC::pend(Interrupt::SW0);
    };
}

pub fn ldma_nfc_take_done() -> Option<NfcReceive> {
    free(|cs| {
        LDMA_TIMER0_DONE.borrow(cs).take()
    })
}

pub fn ldma_nfc_set_next(new_next: NfcReceive) {
    free(|cs| {
        if LDMA_TIMER0_NEXT.borrow(cs).take().is_none() {
            LDMA_TIMER0_NEXT.borrow(cs).set(Some(new_next))
        } else {
            unreachable!("LDMA Timer0 channel next buffer should be switched at this moment")
        }
        if LDMAchTimer0::done() {
            ldma_nfc_switch_buffer(cs);
        }
    })
}

pub fn init_ldma_nfc_buffers() {
    LDMAchTimer0::set_static_cell(Some(ReceivableTIMER::NfcReceive(NfcReceive::new())));
    ldma_nfc_set_next(NfcReceive::new());
}

pub fn purge_ldma_nfc_buffers() {
    free(|cs| {
        LDMA_TIMER0_RECEIVABLE.borrow(cs).take();
        LDMA_TIMER0_NEXT.borrow(cs).take();
        LDMA_TIMER0_DONE.borrow(cs).take();
    })
}

pub struct LDMAchTimer0(Cell<Option<ReceivableTIMER>>);

impl LdmaCh<ReceivableTIMER, CH_TIM0> for LDMAchTimer0 {
    fn init(peripherals: &efm32pg23_fix::Peripherals) {
        peripherals
            .ldmaxbar_s
            .ch7_reqsel()
            .write(|w_reg| unsafe {
                w_reg
                    .sigsel().bits(0) // _LDMAXBAR_CH_REQSEL_SIGSEL_TIMER0CC0
                    .sourcesel().bits(2) // _LDMAXBAR_CH_REQSEL_SOURCESEL_TIMER0
            }
        );
    }
    fn get_static<'a>(cs: &'a cortex_m::interrupt::CriticalSection) -> &'a Self {
        LDMA_TIMER0_RECEIVABLE.borrow(cs)
    }
    fn get_cell<'a>(&'a self) -> &'a Cell<Option<ReceivableTIMER>> {
        &self.0
    }
}

impl Drop for LDMAchTimer0 {
    fn drop(&mut self) {
        self.take();
    }
}

pub enum ReceivableTIMER {
    NfcReceive(NfcReceive),
}

impl ChObjEnum for ReceivableTIMER {
    fn link(&mut self) -> ChLinkData {
        match self {
            ReceivableTIMER::NfcReceive(a) => {
                a.link()
            },
        }
    }
    fn unlink(&mut self) {
        match self {
            ReceivableTIMER::NfcReceive(a) => {
                a.unlink();
            },
        }
    }
}

pub struct NfcReceive { // Boxed data is never moved
    pub buffer: Box<[u16; NFC_BUF_THIRD]>,
    transfer_block: Box<Descriptor>
}

impl core::ops::Deref for NfcReceive {
    type Target = [u16; NFC_BUF_THIRD];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl NfcReceive {
    pub fn new() -> Self {
        let buffer = Box::new([1; NFC_BUF_THIRD]);
        let transfer_block = Box::new(
            Descriptor {
                ctrl: LINK_DESCRIPTORS,
                source: TIMER0_CC0_ICF,
                dest: addr_of!(*buffer) as u32,
                link: 0,
            },
        );

        Self {
            buffer,
            transfer_block
        }
    }

    fn link(&self) -> ChLinkData {
        ChLinkData {
            linkaddr: addr_of!(*self.transfer_block) as u32 >> 2,
            loopcnt: 0,
            ien: true, // to switch buffer
        }
    }

    fn unlink(&self) {}
}
