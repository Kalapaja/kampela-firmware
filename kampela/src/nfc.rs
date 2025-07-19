//! NFC packet collector and decoder

use core::cell::RefCell;

use alloc::borrow::ToOwned;
use cortex_m::interrupt::{Mutex, free};
use nfca_parser::frame::Frame;

use kampela_system::{
    devices::psram::NfcEthSignRequestPsramAccess, if_in_free, in_free, peripherals::ldma_ch_timer::{init_ldma_nfc_buffers, ldma_nfc_set_next, ldma_nfc_take_done, purge_ldma_nfc_buffers}
};

use efm32pg23_fix::interrupt;

use kampela_system::devices::psram::{AddressPsram, ExternalPsram, PsramAccess, psram_read_at_address};
use lt_codes::{decoder_metal::ExternalData, mock_worst_case::DecoderMetal, packet::{Packet, PACKET_SIZE}};
//use substrate_crypto_light::sr25519::PUBLIC_LEN;
use substrate_parser::compacts::find_compact;

pub const FREQ: u16 = 22;
static NFC_COLLECTOR: Mutex<RefCell<NfcCollector>> = Mutex::new(RefCell::new(NfcCollector::Empty));
static NFC_RECEIVED: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

#[interrupt]
fn SW0() {
    free(|cs| {
        let mut collector = NFC_COLLECTOR.borrow(cs).borrow_mut();
        if let Some(done) = ldma_nfc_take_done() {
            if !collector.turn(&(*done.buffer)) {
                ldma_nfc_set_next(done);
            } else {
                purge_ldma_nfc_buffers();
            }
        } else {
            unreachable!("Done buffer should be filled, collection called in LDMA interrupt")
        }
    });
}

fn take_data_if_done() -> Option<ExternalData<AddressPsram>> {
    free(|cs| {
        let mut collector = NFC_COLLECTOR.borrow(cs).borrow_mut();
        if matches!(*collector, NfcCollector::Done(_)) {
            match core::mem::take(&mut *collector) {
                NfcCollector::Done(a) => return Some(a),
                _ => unreachable!()
            }
        } else {
            None
        }
    })
}

pub enum NfcCollector {
    Empty,
    InProgress(DecoderMetal<AddressPsram>),
    Done(ExternalData<AddressPsram>)
}

impl Default for NfcCollector {
    fn default() -> Self {
        Self::Empty
    }
}

impl NfcCollector {
    fn add_packet(&mut self, external_psram: &mut ExternalPsram, nfc_packet: Packet) -> bool {
        match self {
            NfcCollector::Empty => {
                let decoder_metal = DecoderMetal::init(external_psram, nfc_packet).unwrap();
                match decoder_metal.try_read(external_psram) {
                    None => {
                        *self = NfcCollector::InProgress(decoder_metal);
                        false
                    },
                    Some(a) => {
                        *self = NfcCollector::Done(a);
                        true
                    },
                }
            },
            NfcCollector::InProgress(decoder_metal) => {
                decoder_metal.add_packet(external_psram, nfc_packet).unwrap();
                if let Some(a) = decoder_metal.try_read(external_psram) {
                    *self = NfcCollector::Done(a);
                    true
                } else {
                    false
                }
            },
            NfcCollector::Done(_) => { true },
        }
    }

    pub fn turn(&mut self, buffer: &[u16]) -> bool {
        let frames = Frame::process_buffer_miller_skip_tails::<_, FREQ>(
            buffer,
            |frame| {
                if let Frame::Standard(standard_frame) = frame {
                    if standard_frame.len() >= PACKET_SIZE {
                        free(|cs| {
                            let mut received = NFC_RECEIVED.borrow(cs).borrow_mut();
                            *received += 1;
                        });
                        true
                    }
                    else {false}
                } else {false}
            }
        );
    
        for frame in frames.into_iter() {
            if let Frame::Standard(standard_frame) = frame {
                let serialized_packet = standard_frame[standard_frame.len() - PACKET_SIZE..].try_into().expect("static length, always fits");
                if if_in_free(|peripherals| {
                    let mut external_psram = ExternalPsram{peripherals};
                    let packet = Packet::deserialize(serialized_packet);
                    self.add_packet(&mut external_psram, packet)
                }) {
                    return true
                }
            }
            else {unreachable!()}
        }
        false
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

#[derive(Clone)]
pub struct NfcTransactionPsramAccess {
    pub sender_public_key_psram_access: PsramAccess,
    pub call_psram_access: PsramAccess,
    pub extension_psram_access: PsramAccess,
    pub metadata_psram_access: PsramAccess,
    pub genesis_hash_bytes_psram_access: PsramAccess,
}

//TODO: implement more error cases, i.e. old specs
pub enum NfcError {
    InvalidAddress,
}

#[derive(Clone)]
pub enum NfcResult {
    //Transaction(NfcTransactionPsramAccess),
    EthSignRequest(NfcEthSignRequestPsramAccess),
    DisplayAddress,
    Empty,
}

#[derive(Clone)]
pub enum NfcState {
    Operational(usize),
    Done(NfcResult),
}

pub struct NfcReceiver {
    state: NfcState,
}

impl NfcReceiver {
    pub fn new() -> Self {
        init_ldma_nfc_buffers();
        Self {
            state: NfcState::Operational(0),
        }
    }

    fn process(&mut self) -> Option<Result<NfcResult, NfcError>> {
        if let Some(nfc_result) = take_data_if_done() {
            let payload = process_nfc_payload(&nfc_result).unwrap();

            let mut first_byte: Option<u8> = None;
            in_free(|peripherals| {
                first_byte = Some(psram_read_at_address(peripherals, payload.encoded_data.start_address, 1usize).unwrap()[0]);
            });

            match first_byte {
                Some(2) => return Some(Ok(NfcResult::DisplayAddress)),/*
                Some(3) => {
                    let address = payload.encoded_data.start_address.try_shift(1usize).unwrap();
                    let genesis_hash_bytes_psram_access = PsramAccess{start_address: address, total_len: 32usize};

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

                    let mut data_to_sign_psram_access = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact_transaction_1 = find_compact::<u32, PsramAccess, ExternalPsram>(
                            &payload.encoded_data,
                            &mut external_psram,
                            position
                        ).unwrap(); // fix this madness maybe later
                        position = compact_transaction_1.start_next_unit;
                        
                        let compact_transaction_2 = find_compact::<u32, PsramAccess, ExternalPsram>(
                            &payload.encoded_data,
                            &mut external_psram,
                            position
                        ).unwrap();
                        position = compact_transaction_2.start_next_unit;

                        let compact_call = find_compact::<u32, PsramAccess, ExternalPsram>(&
                            payload.encoded_data,
                            &mut external_psram,
                            position
                        ).unwrap();

                        let call_address_to_sign = payload.encoded_data.start_address
                            .try_shift(compact_call.start_next_unit)
                            .unwrap();

                        let extension_address_to_sign = payload.encoded_data.start_address
                            .try_shift(compact_call.start_next_unit + compact_call.compact as usize)
                            .unwrap();
                        let extension_len_to_sign = compact_transaction_2.compact as usize - compact_call.start_next_unit - compact_call.compact as usize + position;

                        let call_to_sign_psram_access = PsramAccess{start_address: call_address_to_sign, total_len: compact_call.compact as usize};
                        let extension_to_sign_psram_access = PsramAccess{start_address: extension_address_to_sign, total_len: extension_len_to_sign};

                        position = compact_transaction_2.start_next_unit + compact_transaction_2.compact as usize;

                        let start_address = payload.encoded_data.start_address.try_shift(position).unwrap();
                        let sender_public_key_psram_accessPUBLIC_LEN = PsramAccess{start_address, total_len: };
                        data_to_sign_psram_access = Some((sender_public_key_psram_access, call_to_sign_psram_access, extension_to_sign_psram_access));
                    });
                    let (sender_public_key_psram_access, call_to_sign_psram_access, extension_to_sign_psram_access) = data_to_sign_psram_access.unwrap();

                    return Some(Ok(NfcResult::Transaction(NfcTransactionPsramAccess{
                        sender_public_key_psram_access,
                        call_psram_access: call_to_sign_psram_access,
                        extension_psram_access: extension_to_sign_psram_access,
                        metadata_psram_access,
                        genesis_hash_bytes_psram_access,
                    })));
                },*/
                Some(4) => {
                    let request_id_address = payload.encoded_data.start_address.try_shift(1usize).unwrap();
                    let request_id = PsramAccess{start_address: request_id_address, total_len: 16};
                    let mut position = 1usize + 16usize;
                    let mut sign_data_option = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact = find_compact::<u32, PsramAccess, ExternalPsram>(&payload.encoded_data, &mut external_psram, position).unwrap();
                        let start_address = payload.encoded_data.start_address.try_shift(compact.start_next_unit.to_owned()).unwrap();
                        sign_data_option = Some(PsramAccess{start_address, total_len: compact.compact as usize});
                        position = compact.start_next_unit + compact.compact as usize;
                    });
                    let sign_data = sign_data_option.unwrap();

                    let start_address = payload.encoded_data.start_address.try_shift(position).unwrap();
                    let data_type = PsramAccess{start_address, total_len: 1};
                    position += 1;

                    let start_address = payload.encoded_data.start_address.try_shift(position).unwrap();
                    let chain_id = PsramAccess{start_address, total_len: 8};
                    position += 8;

                    let mut derivation_path_option = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact = find_compact::<u32, PsramAccess, ExternalPsram>(&payload.encoded_data, &mut external_psram, position).unwrap();
                        let start_address = payload.encoded_data.start_address.try_shift(compact.start_next_unit.to_owned()).unwrap();
                        derivation_path_option = Some(PsramAccess{start_address, total_len: compact.compact as usize});
                        position = compact.start_next_unit + compact.compact as usize;
                    });
                    let derivation_path = derivation_path_option.unwrap();

                    let start_address = payload.encoded_data.start_address.try_shift(position).unwrap();
                    let source_fingerprint = PsramAccess{start_address, total_len: 4};
                    position += 4;
                    
                    let start_address = payload.encoded_data.start_address.try_shift(position).unwrap();
                    let address = PsramAccess{start_address, total_len: 20};
                    position += 20;

                    let mut origin_option = None;
                    in_free(|peripherals| {
                        let mut external_psram = ExternalPsram{peripherals};
                        let compact = find_compact::<u32, PsramAccess, ExternalPsram>(&payload.encoded_data, &mut external_psram, position).unwrap();
                        let start_address = payload.encoded_data.start_address.try_shift(compact.start_next_unit.to_owned()).unwrap();
                        origin_option = Some(PsramAccess{start_address, total_len: compact.compact as usize});
                        position = compact.start_next_unit + compact.compact as usize;
                    });
                    let origin = origin_option.unwrap();
                    
                    return Some(Ok(NfcResult::EthSignRequest(NfcEthSignRequestPsramAccess {
                        request_id,
                        sign_data,
                        data_type,
                        chain_id,
                        derivation_path,
                        source_fingerprint,
                        address,
                        origin
                    })))
                }
                _ => {
                    return Some(Ok(NfcResult::Empty))
                }
            }
        } else {
            None
        }
    }

    pub fn advance(&mut self) -> Result<NfcState, NfcError> {
        //if voltage() < NFC_MIN_VOLTAGE { return None }
        //if !LDMAchTimer0::busy() {return None} // todo: check if no nfc packets were sent

        match self.state {
            NfcState::Operational(_) => {
                let res = self.process();
                match res {
                    Some(r) => {
                        match r {
                            Err(e) => { return Err(e) },
                            Ok(r) => {
                                self.state = NfcState::Done(r);
                            },
                        }
                    },
                    None => {
                        free(|cs| {
                            let i = NFC_RECEIVED.borrow(cs).borrow_mut().to_owned();
                            self.state = NfcState::Operational(i);
                        })
                    },
                }
            },
            _ => {}
        };
        Ok(self.state.clone())
    }
}

impl Drop for NfcReceiver {
    fn drop(&mut self) {
        // receiving should be done at this moment, otherwise will error
        purge_ldma_nfc_buffers();
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
