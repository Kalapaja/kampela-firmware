//! external RAM

use alloc::{format, vec::Vec, string::String};
use efm32pg23_fix::Peripherals;
use crate::peripherals::eusart::*;
use crate::in_free;

pub fn read_from_psram(psram_access: &PsramAccess) -> Vec<u8> {
    let mut bytes_option = None;
    in_free(|peripherals| {
        bytes_option = Some(
            psram_read_at_address(
                peripherals,
                psram_access.start_address,
                psram_access.total_len
            ).unwrap()
        );
    });
    
    bytes_option.unwrap()
}

pub fn psram_reset(peripherals: &mut Peripherals) {
    deselect_psram(&mut peripherals.GPIO_S);
    select_psram(&mut peripherals.GPIO_S);
    psram_write_read_byte(peripherals, PSRAM_RESET_ENABLE);
    deselect_psram(&mut peripherals.GPIO_S);
    select_psram(&mut peripherals.GPIO_S);
    psram_write_read_byte(peripherals, PSRAM_RESET);
    deselect_psram(&mut peripherals.GPIO_S);
}

pub fn psram_write_read_byte(peripherals: &mut Peripherals, byte: u8) -> u8 {
    while peripherals.EUSART2_S.status.read().txfl().bit_is_clear() {}
    peripherals.EUSART2_S.txdata.write({|w_reg|
        w_reg
            // EUSART tx and rx are u16,
            // single byte is used here because of the commands,
            // setting used is `.databits().eight()`
            .txdata().variant(byte as u16)
    });
    while peripherals.EUSART2_S.status.read().rxfl().bit_is_clear() {}
    peripherals.EUSART2_S.rxdata.read().rxdata().bits().try_into().expect("configured frame for 8 data bits")
}

/// PSRAM dummy command, to send a new item in rx.
///
/// Could have switched into autotx mode instead.
pub const PSRAM_DUMMY: u8 = 0xff;

pub fn psram_read_id(peripherals: &mut Peripherals) -> [u8; ID_LEN] {
    select_psram(&mut peripherals.GPIO_S);
    psram_write_read_byte(peripherals, PSRAM_READ_ID);
    psram_write_slice(peripherals, &[PSRAM_DUMMY; ADDR_LEN]);
    psram_read_vec(peripherals, ID_LEN).try_into().expect("static length, always fits")
}

pub fn psram_write_slice(peripherals: &mut Peripherals, slice: &[u8]) {
    for byte in slice.iter() {
        psram_write_read_byte(peripherals, *byte);
    }
}

pub fn psram_read_vec(peripherals: &mut Peripherals, len: usize) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(len);
    for _i in 0..len {
        out.push(psram_write_read_byte(peripherals, PSRAM_DUMMY))
    }
    out
}

/// PSRAM commands from manual
pub const PSRAM_RESET_ENABLE: u8 = 0x66;
pub const PSRAM_RESET: u8 = 0x99;
pub const PSRAM_READ_ID: u8 = 0x9f;
pub const PSRAM_READ: u8 = 0x03;
pub const PSRAM_WRITE: u8 = 0x02;

pub const ID_LEN: usize = 3;
pub const ADDR_LEN: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct AddressPsram([u8; ADDR_LEN]);

impl AddressPsram {
    pub fn new(into_inner: u32) -> Result<Self, MemoryError> {
        let bytes = into_inner.to_be_bytes();
        if (bytes[0] != 0) | (bytes[1] > 0x8f) {Err(MemoryError::Overflow)}
        else {Ok(Self(bytes[1..].try_into().expect("static length, always fits")))}
    }
    pub fn zero() -> Self {
        Self([0; ADDR_LEN])
    }
    pub fn inner(&self) -> [u8; ADDR_LEN] {
        self.0
    }
    pub fn as_u32(&self) -> u32 {
        let mut be_bytes = [0;4];
        be_bytes[1..].copy_from_slice(&self.inner());
        u32::from_be_bytes(be_bytes)
    }
    pub fn try_shift(&self, position: usize) -> Result<Self, MemoryError> {
        let new_inner = self.as_u32() + position as u32;
        Self::new(new_inner)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryError {
    Overflow,
    ReadTooLarge,
    TypeInfoDamaged{id: u32},
    WriteTooLarge,
}

/// Reads data with wrapping, i.e. when the page ends, starts to read at the
/// same page beginning.
///
/// Slice length should be checked elsewhere to be sufficiently low to read only
/// data from the address going forward.
pub fn psram_read_at_address_native(peripherals: &mut Peripherals, address: AddressPsram, len: usize) -> Vec<u8> {
    psram_reset(peripherals);
    psram_read_at_address_helper(peripherals, address, len)
}

fn psram_read_at_address_helper(peripherals: &mut Peripherals, address: AddressPsram, len: usize) -> Vec<u8> {
    select_psram(&mut peripherals.GPIO_S);
    psram_write_read_byte(peripherals, PSRAM_READ);
    psram_write_slice(peripherals, &address.inner());
    let out = psram_read_vec(peripherals, len);
    deselect_psram(&mut peripherals.GPIO_S);
    out
}
pub fn psram_read_at_address(peripherals: &mut Peripherals, address: AddressPsram, len: usize) -> Result<Vec<u8>, MemoryError> {
    psram_reset(peripherals);
    
    let start = address.as_u32();

    if start + len as u32 > PSRAM_TOTAL_SIZE {return Err(MemoryError::ReadTooLarge)}
    let mut out = Vec::with_capacity(len);

    let space_left_on_page = PSRAM_PAGE_SIZE - start%PSRAM_PAGE_SIZE;
    if len as u32 <= space_left_on_page {
        out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, len));
    }
    else {
        out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, space_left_on_page as usize));
        let full_pages = (len as u32 - space_left_on_page)/PSRAM_PAGE_SIZE;
        for i in 0..full_pages {
            let full_page_start = (start/PSRAM_PAGE_SIZE + 1 + i) * PSRAM_PAGE_SIZE;
            let address = AddressPsram::new(full_page_start).expect("checked that length does not overflow");
            out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, PSRAM_PAGE_SIZE as usize));
        }
        let last_page_start = (start/PSRAM_PAGE_SIZE + 1 + full_pages) * PSRAM_PAGE_SIZE;
        let address = AddressPsram::new(last_page_start).expect("checked that length does not overflow");
        let tail_len = len - (space_left_on_page + full_pages*PSRAM_PAGE_SIZE) as usize;
        out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, tail_len));
    }
    Ok(out)
}
/// Writes data with wrapping, i.e. when the page ends, starts to write at the
/// same page beginning.
///
/// Slice length should be checked elsewhere to be sufficiently low to fit on
/// page without wrapping.
pub fn psram_write_at_address_native(peripherals: &mut Peripherals, address: AddressPsram, slice: &[u8]) {
    psram_reset(peripherals);
    psram_write_at_address_helper(peripherals, address, slice);
}

/// Helper function, without reset at start.
///
/// Use only as a part of function with reset.
fn psram_write_at_address_helper(peripherals: &mut Peripherals, address: AddressPsram, slice: &[u8]) {
    select_psram(&mut peripherals.GPIO_S);
    psram_write_read_byte(peripherals, PSRAM_WRITE);
    psram_write_slice(peripherals, &address.inner());
    psram_write_slice(peripherals, slice);
    deselect_psram(&mut peripherals.GPIO_S);
}
/// Write at address seamlessly, i.e. without wrapping.
///
/// Each new byte is written to the next address.
pub fn psram_write_at_address(peripherals: &mut Peripherals, address: AddressPsram, slice: &[u8]) -> Result<(), MemoryError> {
    psram_reset(peripherals);
    
    let start = address.as_u32();
    let slice_len = slice.len() as u32;

    if start + slice_len > PSRAM_TOTAL_SIZE {return Err(MemoryError::WriteTooLarge)}

    let space_left_on_page = PSRAM_PAGE_SIZE - start%PSRAM_PAGE_SIZE;
    if slice_len <= space_left_on_page {
        psram_write_at_address_helper(peripherals, address, slice);
    }
    else {
        psram_write_at_address_helper(peripherals, address, &slice[..space_left_on_page as usize]);
        let full_pages = (slice_len - space_left_on_page)/PSRAM_PAGE_SIZE;
        for i in 0..full_pages {
            let full_page_start = (start/PSRAM_PAGE_SIZE + 1 + i) * PSRAM_PAGE_SIZE;
            let address = AddressPsram::new(full_page_start).expect("checked that length does not overflow");
            let slice_start = (space_left_on_page + i*PSRAM_PAGE_SIZE) as usize;
            let slice_end = slice_start + PSRAM_PAGE_SIZE as usize;
            psram_write_at_address_helper(peripherals, address, &slice[slice_start..slice_end]);
        }
        let last_page_start = (start/PSRAM_PAGE_SIZE + 1 + full_pages) * PSRAM_PAGE_SIZE;
        let address = AddressPsram::new(last_page_start).expect("checked that length does not overflow");
        let slice_start = (space_left_on_page + full_pages*PSRAM_PAGE_SIZE) as usize;
        psram_write_at_address_helper(peripherals, address, &slice[slice_start..]);
    }
    Ok(())
}
/// PSRAM is *paged*, with data in pages wrapped at page end.
pub const PSRAM_PAGE_SIZE: u32 = 1024;

/// PSRAM total size, 2^23 byte.
///
/// Limits maximum address available to `AddressPsram([0x8f, ff, ff])`.
pub const PSRAM_TOTAL_SIZE: u32 = 67_108_864;

#[derive(Debug)]
pub struct PsramAccess {
    pub start_address: AddressPsram,
    pub total_len: usize,
}
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use external_memory_tools::{AddressableBuffer, BufferError, ExternalMemory};

pub struct ExternalPsram<'a> {
    pub peripherals: &'a mut Peripherals,
}

impl <'a> Debug for ExternalPsram<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ExternalPsram")
    }
}

impl <'a> ExternalMemory for ExternalPsram<'a> {
    type ExternalMemoryError = MemoryError;
}
impl MemoryError {
    pub fn error_text(&self) -> String {
        match &self {
            MemoryError::Overflow => String::from("Attempted to read at address that does not exist."),
            MemoryError::ReadTooLarge => String::from("Attempted to read at address that does not exist."),
            MemoryError::TypeInfoDamaged{id} => format!("Type information for type id {id} in metadata is damaged."),
            MemoryError::WriteTooLarge => String::from("Attempted to write into PSRAM data that exceeds available space."),
        }
    }
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.error_text())
    }
}
impl <'a> AddressableBuffer<ExternalPsram<'a>> for PsramAccess {
    type ReadBuffer = Vec<u8>;
    fn total_len(&self) -> usize {
        self.total_len
    }
    fn read_slice(&self, ext_memory: &mut ExternalPsram<'a>, position: usize, len: usize) -> Result<Self::ReadBuffer, BufferError<ExternalPsram<'a>>> {
        if self.total_len() < position {return Err(BufferError::OutOfRange { position, total_length: self.total_len() })}
        if self.total_len() < (position + len) {return Err(BufferError::DataTooShort { position: self.total_len(), minimal_length: position + len - self.total_len() })}
        let address = self.start_address.try_shift(position).map_err(BufferError::External)?;
        psram_read_at_address(ext_memory.peripherals, address, len).map_err(BufferError::External)
    }
    fn limit_length(&self, new_len: usize) -> Result<Self, BufferError<ExternalPsram<'a>>> {
        if new_len > self.total_len {Err(BufferError::DataTooShort { position: 0, minimal_length: new_len })}
        else {Ok(PsramAccess {
            start_address: self.start_address,
            total_len: new_len,
        })}
    }
}

use lt_codes::decoder_metal::ExternalAddress;

impl ExternalAddress for AddressPsram {
    fn zero() -> Self {
        AddressPsram::zero()
    }
    fn shift(&mut self, position: usize) {
        *self = self.try_shift(position).unwrap(); //TODO
    }
}

impl <'a> lt_codes::decoder_metal::ExternalMemory<AddressPsram> for ExternalPsram<'a> {

    fn write_external(&mut self, address: &AddressPsram, data: &[u8]) {
         psram_write_at_address(self.peripherals, *address, data).unwrap() //TODO
    }
    fn read_external(&mut self, address: &AddressPsram, len: usize) -> Vec<u8> {
         psram_read_at_address(self.peripherals, *address, len).unwrap() //TODO
    }
}
