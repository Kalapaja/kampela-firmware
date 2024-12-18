//! external RAM

use alloc::{format, vec::Vec, string::String};
use primitive_types::H256;
use efm32pg23_fix::Peripherals;
use crate::peripherals::eusart::*;
use substrate_parser::{cards::{Call, ExtendedData}, decode_as_call_unmarked, decode_extensions_unmarked};
use crate::in_free;

pub fn psram_decode_call(call_psram_access: &PsramAccess, metadata_psram_access: &PsramAccess) -> (Call, ShortSpecs, String) {
    let call_data = read_from_psram(call_psram_access);

    let (
        checked_metadata_metal,
        specs,
        spec_name,
    ) = read_checked_metadata_metal(metadata_psram_access);

    let mut decoded_call_option = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let mut decoding_postition = 0;
        let decoded_call = decode_as_call_unmarked(
            &call_data.as_ref(),
            &mut decoding_postition,
            &mut external_psram,
            &checked_metadata_metal,
        ).unwrap();

        decoded_call_option = Some(decoded_call);
    });
    (
        decoded_call_option.unwrap(),
        specs,
        spec_name,
    )
}

pub fn psram_decode_extension(
    extension_psram_access: &PsramAccess,
    metadata_psram_access: &PsramAccess,
    genesis_hash_bytes_psram_access: &PsramAccess,
) -> (Vec<ExtendedData>, ShortSpecs, String) {
    let extension_data = read_from_psram(extension_psram_access);
    
    let (
        checked_metadata_metal,
        specs,
        spec_name
    ) = read_checked_metadata_metal(metadata_psram_access);

    let genesis_hash = H256(
        read_from_psram(genesis_hash_bytes_psram_access)
            .try_into()
            .expect("static size")
    );

    let mut decoded_extension_option = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let mut decoding_postition = 0;
        let decoded_extension = decode_extensions_unmarked(
            &extension_data.as_ref(),
            &mut decoding_postition,
            &mut external_psram,
            &checked_metadata_metal,
            Some(genesis_hash),
        ).unwrap();

        decoded_extension_option = Some(decoded_extension);
    });
    (
        decoded_extension_option.unwrap(),
        specs,
        spec_name,
    )
}

fn read_checked_metadata_metal(metadata_psram_access: &PsramAccess) -> (CheckedMetadataMetal, ShortSpecs, String) {
    let mut checked_metadata_metal_option = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        checked_metadata_metal_option = Some(
            CheckedMetadataMetal::from(
                metadata_psram_access,
                &mut external_psram
            ).unwrap()
        );
    });

    let checked_metadata_metal = checked_metadata_metal_option.unwrap();
    let specs = checked_metadata_metal.to_specs();
    let spec_name = checked_metadata_metal.spec_name_version.spec_name.to_owned();
    (
        checked_metadata_metal,
        specs,
        spec_name
    )
}

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
use core::{any::TypeId, fmt::{Debug, Display, Formatter, Result as FmtResult}};
use alloc::borrow::ToOwned;

use external_memory_tools::{AddressableBuffer, BufferError, ExternalMemory};
use parity_scale_codec::{Decode, DecodeAll, Encode};
use substrate_parser::{AsMetadata, ResolveType, ShortSpecs, compacts::find_compact, error::{RegistryError, RegistryInternalError}, traits::{SignedExtensionMetadata, SpecNameVersion}};
use scale_info::{form::PortableForm, interner::UntrackedSymbol, Type};

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

#[derive(Clone, Debug)]
pub struct MetalRegistry {
    pub start_address: AddressPsram,
    pub registry: Vec<EntryPsram>,
}

#[derive(Clone, Debug)]
pub struct EntryPsram {
    pub id: u32,
    pub position: usize,
    pub entry_len: usize,
}

impl <'a> ResolveType<ExternalPsram<'a>> for MetalRegistry {
    fn resolve_ty(&self, id: u32, ext_memory: &mut ExternalPsram<'a>) -> Result<Type<PortableForm>, RegistryError<ExternalPsram<'a>>> {
        for entry_psram in self.registry.iter() {
            if entry_psram.id == id {
                let address = self.start_address.try_shift(entry_psram.position).map_err(RegistryError::External)?;
                let encoded_type_data = psram_read_at_address(ext_memory.peripherals, address, entry_psram.entry_len).map_err(RegistryError::External)?;
                let ty = Type::<PortableForm>::decode_all(&mut &encoded_type_data[..]).map_err(|_| RegistryError::External(MemoryError::TypeInfoDamaged{id}))?;
                return Ok(ty)
            }
        }
        Err(RegistryError::Internal(RegistryInternalError::TypeNotResolved { id }))
    }
}

#[derive(Debug)]
pub struct CheckedMetadataMetal {
    pub types: MetalRegistry,
    pub call_ty: UntrackedSymbol<TypeId>,
    pub signed_extensions: Vec<SignedExtensionMetadata>,
    pub spec_name_version: SpecNameVersion,
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

#[derive(Debug, Decode, Encode)]
pub struct CheckedMeadataMetalTail {
    pub call_ty: UntrackedSymbol<TypeId>,
    pub signed_extensions: Vec<SignedExtensionMetadata>,
    pub spec_name_version: SpecNameVersion,
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

impl <'a> AsMetadata<ExternalPsram<'a>> for CheckedMetadataMetal {
    type TypeRegistry = MetalRegistry;
    type MetaStructureError = NoEntries;
    fn types(&self) -> Self::TypeRegistry {
        self.types.to_owned()
    }
    fn spec_name_version(&self) -> Result<SpecNameVersion, Self::MetaStructureError> {
        Ok(self.spec_name_version.to_owned())
    }
    fn call_ty(&self) -> Result<UntrackedSymbol<TypeId>, Self::MetaStructureError> {
        Ok(self.call_ty.to_owned())
    }
    fn signed_extensions(&self) -> Result<Vec<SignedExtensionMetadata>, Self::MetaStructureError> {
        Ok(self.signed_extensions.to_owned())
    }
}

/// Empty error enum, for cases with fault-free memory access.
#[derive(Debug, Eq, PartialEq)]
pub enum NoEntries {}

impl Display for NoEntries {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "")
    }
}

fn force_decode_at<T: Decode>(psram_data: &PsramAccess, ext_memory: &mut ExternalPsram<'_>, start_position: usize, err_at: ReceivedMetadataError) -> Result<(T, usize), ReceivedMetadataError> {
    let mut data = Vec::with_capacity(psram_data.total_len - start_position);
    let mut out: Option<(T, usize)> = None;
    for i in 0..psram_data.total_len - start_position {
        let address = psram_data.start_address.try_shift(start_position+i).map_err(|_| err_at.to_owned())?;
        let byte = psram_read_at_address(ext_memory.peripherals, address, 1usize).map_err(|_| err_at.to_owned())?[0];
        data.push(byte);
        if let Ok(a) = T::decode(&mut &data[..]) {
            out = Some((a, i+1));
            break;
        }
    }
    match out {
        Some(a) => Ok(a),
        None => Err(err_at),
    }
}
impl <'a> CheckedMetadataMetal {
    /// Assume here that the metadata is received as SCALE-encoded
    /// `ShortMetadata` with known length.
    ///
    /// Provided `PsramAccess` corresponds to whole encoded metadata.
    pub fn from(psram_data: &PsramAccess, ext_memory: &mut ExternalPsram<'a>) -> Result<Self, ReceivedMetadataError> {
        
        let mut position = 0usize;

        // Metadata starts with types registry, a vec of Type descriptors.
        // Search for compact, the number of `PortableType` entries to follow.
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram<'a>>(psram_data, ext_memory, position).map_err(|_| ReceivedMetadataError::Format)?;

        let types_set_len = found_compact.compact;
        let mut registry: Vec<EntryPsram> = Vec::with_capacity(types_set_len as usize);
        position = found_compact.start_next_unit;
        
        for _entry_number in 0..types_set_len {
            // Each `PortableType` starts with compact of the id.
            let entry_number_compact = find_compact::<u32, PsramAccess, ExternalPsram<'a>>(psram_data, ext_memory, position).map_err(|_| ReceivedMetadataError::Format)?;
            position = entry_number_compact.start_next_unit;
            
            // And is followed by encoded `Type<PortableForm>` entry.
            let (_ty, entry_len) = force_decode_at::<Type<PortableForm>>(psram_data, ext_memory, position, ReceivedMetadataError::Format)?;

            registry.push(EntryPsram{id: entry_number_compact.compact, position, entry_len});

            position += entry_len;
        }

        let types = MetalRegistry {
            start_address: psram_data.start_address, 
            registry,
        };

        // The rest corresponds to `CheckedMeadataMetalTail`

        let tail_data = psram_data.read_slice(ext_memory, position, psram_data.total_len - position).map_err(|_| ReceivedMetadataError::Format)?;
        let tail = CheckedMeadataMetalTail::decode_all(&mut &tail_data[..]).map_err(|_| ReceivedMetadataError::Format)?;

        Ok(CheckedMetadataMetal{
            types,
            call_ty: tail.call_ty,
            signed_extensions: tail.signed_extensions,
            spec_name_version: tail.spec_name_version,
            base58prefix: tail.base58prefix,
            decimals: tail.decimals,
            unit: tail.unit,
        })
    }

    pub fn to_specs(&self) -> ShortSpecs {
        ShortSpecs {
            base58prefix: self.base58prefix,
            decimals: self.decimals,
            unit: self.unit.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ReceivedMetadataError {
    Format,
//    Memory(MemoryError),
//    UnableToDecode,
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
