//! NFC packet collector and decoder

use nfca_parser::frame::Frame;
//use alloc::vec::Vec;
use alloc::{borrow::ToOwned, format, string::String, vec::Vec};

use kampela_system::{
    PERIPHERALS, in_free, BUF_THIRD, CH_TIM0,
};
use cortex_m::interrupt::free;
use crate::BUFFER_STATUS;
use efm32pg23_fix::{NVIC,Interrupt};

use kampela_system::devices::psram::{AddressPsram, ExternalPsram, PsramAccess, CheckedMetadataMetal, psram_read_at_address};
use lt_codes::{decoder_metal::ExternalData, mock_worst_case::DecoderMetal, packet::{Packet, PACKET_SIZE}};
// use substrate_parser::compacts::find_compact;
use substrate_parser::{MarkedData, compacts::find_compact, parse_transaction_unmarked, TransactionUnmarkedParsed, ShortSpecs};
use schnorrkel::{
    context::attach_rng,
    keys::Keypair,
    signing_context,
};

use core::ops::DerefMut;

use primitive_types::H256;
use parity_scale_codec::Decode;

pub const FREQ: u16 = 22;
const NFC_MIN_VOLTAGE: i32 = 4000;

#[derive(Clone, Debug)]
pub enum BufferStatus {
    R0W1,
    R0Wh,
    R1W2,
    R1Wh,
    R2W0,
    R2Wh,
    RhW0,
    RhW1,
    RhW2,
}

#[derive(Debug)]
pub enum BufRegion {
    Reg0,
    Reg1,
    Reg2,
}


#[derive(Debug)]
pub enum BufferError {
    UnexpectedIfDone7,
    UnexpectedReadDone,
}

impl BufferStatus {
    pub fn new() -> Self {
        Self::RhW0
    }
    pub fn pass_if_done7(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::R0Wh,
            Self::R0Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R1W2 => Self::R1Wh,
            Self::R1Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R2W0 => Self::R2Wh,
            Self::R2Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::RhW0 => Self::R0W1,
            Self::RhW1 => Self::R1W2,
            Self::RhW2 => Self::R2W0,
        };
        *self = new_self;
        Ok(())
    }
    pub fn pass_read_done(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::RhW1,
            Self::R0Wh => Self::R1W2,
            Self::R1W2 => Self::RhW2,
            Self::R1Wh => Self::R2W0,
            Self::R2W0 => Self::RhW0,
            Self::R2Wh => Self::R0W1,
            Self::RhW0 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW1 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW2 => return Err(BufferError::UnexpectedReadDone),
        };
        *self = new_self;
        Ok(())
    }
    pub fn read_from(&self) -> Option<BufRegion> {
        match self {
            Self::R0W1 => Some(BufRegion::Reg0),
            Self::R0Wh => Some(BufRegion::Reg0),
            Self::R1W2 => Some(BufRegion::Reg1),
            Self::R1Wh => Some(BufRegion::Reg1),
            Self::R2W0 => Some(BufRegion::Reg2),
            Self::R2Wh => Some(BufRegion::Reg2),
            Self::RhW0 => None,
            Self::RhW1 => None,
            Self::RhW2 => None,
        }
    }
    pub fn is_write_halted(&self) -> bool {
        match self {
            Self::R0W1 => false,
            Self::R0Wh => true,
            Self::R1W2 => false,
            Self::R1Wh => true,
            Self::R2W0 => false,
            Self::R2Wh => true,
            Self::RhW0 => false,
            Self::RhW1 => false,
            Self::RhW2 => false,
        }
    }
}

pub fn turn_nfc_collector_correctly(collector: &mut NfcCollector, nfc_buffer: &[u16; 3*BUF_THIRD]) {
    let mut read_from = None;
    free(|cs| {
        let buffer_status = BUFFER_STATUS.borrow(cs).borrow();
        read_from = buffer_status.read_from();
    });
    let decoder_input = match read_from {
        Some(BufRegion::Reg0) => &nfc_buffer[..BUF_THIRD],
        Some(BufRegion::Reg1) => &nfc_buffer[BUF_THIRD..2*BUF_THIRD],
        Some(BufRegion::Reg2) => &nfc_buffer[2*BUF_THIRD..],
        None => return,
    };
    let frames = Frame::process_buffer_miller_skip_tails::<_, FREQ>(decoder_input, |frame| frame_selected(&frame));

    for frame in frames.into_iter() {
        if let Frame::Standard(standard_frame) = frame {
            let serialized_packet = standard_frame[standard_frame.len() - PACKET_SIZE..].try_into().expect("static length, always fits");
            in_free(|peripherals| {
                let mut external_psram = ExternalPsram{peripherals};
                let packet = Packet::deserialize(serialized_packet);
                collector.add_packet(&mut external_psram, packet);
            });
        }
        else {unreachable!()}
    }

    free(|cs| {
        let mut buffer_status = BUFFER_STATUS.borrow(cs).borrow_mut();
        let was_write_halted = buffer_status.is_write_halted();
        buffer_status.pass_read_done().expect("to do");
        if was_write_halted & ! buffer_status.is_write_halted() {
            if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(1 << CH_TIM0));
            }
            else {panic!("can not borrow peripherals, buffer_status: {:?}, got some new frames", buffer_status)}
        }
    });
}

fn frame_selected(frame: &Frame) -> bool {
    if let Frame::Standard(standard_frame) = frame {
        if standard_frame.len() >= PACKET_SIZE {true}
        else {false}
    }
    else {false}
}

pub enum NfcCollector {
    Empty,
    InProgress(DecoderMetal<AddressPsram>),
    Done(ExternalData<AddressPsram>)
}

impl NfcCollector {
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn add_packet(&mut self, external_psram: &mut ExternalPsram, nfc_packet: Packet) {
        match self {
            NfcCollector::Empty => {
                let decoder_metal = DecoderMetal::init(external_psram, nfc_packet).unwrap();
                match decoder_metal.try_read(external_psram) {
                    None => *self = NfcCollector::InProgress(decoder_metal),
                    Some(a) => *self = NfcCollector::Done(a),
                }
            },
            NfcCollector::InProgress(decoder_metal) => {
                decoder_metal.add_packet(external_psram, nfc_packet).unwrap();
                if let Some(a) = decoder_metal.try_read(external_psram) {
                    *self = NfcCollector::Done(a);
                }
            },
            NfcCollector::Done(_) => {},
        }
    }
}

#[derive(Debug)]
pub enum NfcPayloadError {
    AccessOnPayload,
//    AccessOnPublicKey,
//    AccessOnSignature,
//    ExcessData,
//    NoCompactPayload,
//    NoCompactPublicKey,
//    NoCompactSignature,
}

#[derive(Debug)]
pub struct TransferDataReceived {
    pub encoded_data: PsramAccess,
//    pub companion_signature: Vec<u8>,
//    pub companion_public_key: Vec<u8>,
}

pub fn process_nfc_payload(completed_collector: &ExternalData<AddressPsram>) -> Result<TransferDataReceived, NfcPayloadError> {
    let psram_data = PsramAccess {
        start_address: completed_collector.start_address.clone(),
        total_len: completed_collector.len,
    };

    let mut position = 0usize; // *relative* position in PsramAccess!

    let mut try_encoded_data = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram>(&psram_data, &mut external_psram, position).unwrap(); //.map_err(|_| NfcPayloadError::NoCompactPayload)?;
        let start_address = completed_collector.start_address.try_shift(found_compact.start_next_unit).unwrap();
        try_encoded_data = Some(PsramAccess {
            start_address,
            total_len: found_compact.compact as usize,
        });
        position = found_compact.start_next_unit + found_compact.compact as usize;
    });
    let encoded_data = match try_encoded_data {
        Some(a) => a,
        None => return Err(NfcPayloadError::AccessOnPayload),
    };
    Ok(TransferDataReceived{
        encoded_data,
    })
/*
    let mut try_companion_signature = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram>(&psram_data, &mut external_psram, position).unwrap(); //.map_err(|_| NfcPayloadError::NoCompactSignature)?;
        let start_address = completed_collector.start_address.try_shift(found_compact.start_next_unit).unwrap();
        let signature_data = psram_read_at_address(external_psram.peripherals, start_address, found_compact.compact as usize).unwrap(); //.map_err(|_| NfcPayloadError::AccessOnSignature)?;
        try_companion_signature = Some(signature_data);
        position = found_compact.start_next_unit + found_compact.compact as usize;
    });
    let companion_signature = match try_companion_signature {
        Some(a) => a,
        None => return Err(NfcPayloadError::AccessOnSignature),
    };

    let mut try_companion_public_key = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram>(&psram_data, &mut external_psram, position).unwrap(); //.map_err(|_| NfcPayloadError::NoCompactSignature)?;
        let start_address = completed_collector.start_address.try_shift(found_compact.start_next_unit).unwrap();
        let public_key_data = psram_read_at_address(external_psram.peripherals, start_address, found_compact.compact as usize).unwrap(); //.map_err(|_| NfcPayloadError::AccessOnSignature)?;
        try_companion_public_key = Some(public_key_data);
        position = found_compact.start_next_unit + found_compact.compact as usize;
    });
    let companion_public_key = match try_companion_public_key {
        Some(a) => a,
        None => return Err(NfcPayloadError::AccessOnPublicKey),
    };

    if position != psram_data.total_len {
        panic!("after decoding position not matching total length, position: {position}, total_len: {}", psram_data.total_len);
        //Err(NfcPayloadError::ExcessData)
    }
    else {Ok(TransferDataReceived{
        encoded_data,
        companion_signature,
        companion_public_key
    })}
*/
}




pub struct NfcTransaction {
    pub decoded_transaction: TransactionUnmarkedParsed,
    pub data_to_sign: Vec<u8>,
    pub specs: ShortSpecs,
    pub spec_name: String,
}

pub enum NfcResult {
    Transaction(NfcTransaction),
    DisplayAddress,
    KampelaStop,
}

enum NfcState {
    Operational,
    Done,
}


pub struct NfcReceiver <'a> {
    buffer: &'a [u16; 3*BUF_THIRD],
    collector: NfcCollector,
    state: NfcState,
    public_memory: [u8; 32],
}

impl <'a> NfcReceiver<'a> {


    pub fn new(nfc_buffer: &'a [u16; 3*BUF_THIRD], public_memory: [u8; 32]) -> Self {
        Self {
            buffer: nfc_buffer,
            collector: NfcCollector::new(),
            state: NfcState::Operational,
            public_memory: public_memory,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let NfcCollector::Empty = self.collector {
            return true;
        }
        false
    }


    fn process(&mut self) -> Option<NfcResult> {
        turn_nfc_collector_correctly(&mut self.collector, self.buffer);

        if let NfcCollector::Done(ref a) = self.collector {
            NVIC::mask(Interrupt::LDMA);
            let payload = process_nfc_payload(a).unwrap();

            let mut first_byte: Option<u8> = None;
            in_free(|peripherals| {
                first_byte = Some(psram_read_at_address(peripherals, payload.encoded_data.start_address, 1usize).unwrap()[0]);
            });

            match first_byte {
                Some(0) => return Some(NfcResult::KampelaStop),
                Some(2) => return Some(NfcResult::DisplayAddress),
                Some(3) => {
                    let mut genesis_hash_bytes_option = None;
                    in_free(|peripherals| {
                        let address = payload.encoded_data.start_address.try_shift(1usize).unwrap();
                        genesis_hash_bytes_option = Some(psram_read_at_address(peripherals, address, 32usize).unwrap());
                    });
                    let genesis_hash = H256(genesis_hash_bytes_option.unwrap().try_into().expect("static size"));

                    let mut metadata_psram_access_option = None;
                    let mut position = 1usize + 32usize;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact_meta = find_compact::<u32, PsramAccess, ExternalPsram>(&payload.encoded_data, &mut external_psram, position).unwrap();
                        let start_address = payload.encoded_data.start_address.try_shift(compact_meta.start_next_unit).unwrap();
                        metadata_psram_access_option = Some(PsramAccess{start_address, total_len: compact_meta.compact as usize});
                        position = compact_meta.start_next_unit + compact_meta.compact as usize;
                    });
                    let metadata_psram_access = metadata_psram_access_option.unwrap();

                    let mut checked_metadata_metal_option = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        checked_metadata_metal_option = Some(CheckedMetadataMetal::from(&metadata_psram_access, &mut external_psram).unwrap());
                    });
                    let checked_metadata_metal = checked_metadata_metal_option.unwrap();

                    let mut signable_transaction_option = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact_transaction_1 = find_compact::<u32, PsramAccess, ExternalPsram>(&payload.encoded_data, &mut external_psram, position).unwrap(); // fix this madness maybe later
                        position = compact_transaction_1.start_next_unit;
                        let compact_transaction_2 = find_compact::<u32, PsramAccess, ExternalPsram>(&payload.encoded_data, &mut external_psram, position).unwrap();
                        position = compact_transaction_2.start_next_unit;

                        let start_address = payload.encoded_data.start_address.try_shift(compact_transaction_2.start_next_unit).unwrap();

                        let compact_call = find_compact::<u32, PsramAccess, ExternalPsram>(&payload.encoded_data, &mut external_psram, position).unwrap();
                        let start_address_to_sign = payload.encoded_data.start_address.try_shift(compact_call.start_next_unit).unwrap();
                        let total_len_to_sign = compact_transaction_2.compact as usize - compact_call.start_next_unit + position;

                        let data_to_sign = psram_read_at_address(peripherals, start_address_to_sign, total_len_to_sign).unwrap();

                        signable_transaction_option = Some(data_to_sign);
                        position = compact_transaction_2.start_next_unit + compact_transaction_2.compact as usize;
                    });
                    let data_to_sign = signable_transaction_option.unwrap();


                    in_free(|peripherals| {
                        let start_address = payload.encoded_data.start_address.try_shift(position).unwrap();
                        let public_key = psram_read_at_address(peripherals, start_address, 33usize).unwrap();
                        // TODO: check address differently
                        assert!(public_key.starts_with(&[1u8]) & (public_key[1..] == self.public_memory), "Invalid crypto algorithm requested");
                    });

                    let mut got_transaction_no_data = None;
                    in_free(|peripherals| {

                        let mut external_psram = ExternalPsram{peripherals};

                        let decoded_transaction = parse_transaction_unmarked(
                            &data_to_sign.as_ref(),
                            &mut external_psram,
                            &checked_metadata_metal,
                            genesis_hash
                        ).unwrap();

                        got_transaction_no_data = Some((decoded_transaction, checked_metadata_metal.to_specs(), checked_metadata_metal.spec_name_version.spec_name.to_owned()));
                    });
                    let (decoded_transaction, specs, spec_name) = got_transaction_no_data.unwrap();

                    return Some(NfcResult::Transaction(NfcTransaction{
                        decoded_transaction,
                        data_to_sign,
                        specs,
                        spec_name,
                    }));
                },
                _ => {
                    self.collector = NfcCollector::new();
                }
            }
        }
        None

    }

    pub fn advance(&mut self, voltage: i32) -> Option<NfcResult> {
        if (voltage < NFC_MIN_VOLTAGE) { return None; }
        match self.state {
            NfcState::Operational => {
                let res = self.process();
                if res.is_some() {
                    self.state = NfcState::Done;
                }
                res
            },
            NfcState::Done => { None }
        }
    }
}



// if got_transaction.is_some() {

//     let transaction = got_transaction.unwrap();
//     let context = signing_context(SIGNING_CTX);
//     let signature = pair_derived.sign(attach_rng(context.bytes(&transaction.2), &mut SeRng{}));
//     let mut signature_with_id: [u8; 65] = [1; 65];
//     signature_with_id[1..].copy_from_slice(&signature.to_bytes());
//     let signature_into_qr: [u8; 130] = hex::encode(signature_with_id).into_bytes().try_into().expect("static known length");

//     ui.handle_rx(transaction.0, transaction.1, signature_into_qr);

//     break
// }
// None
