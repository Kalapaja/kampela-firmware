//! Everything high-level related to interfacing with user

use nalgebra::{Affine2, OMatrix, Point2, RowVector3};
use alloc::{borrow::ToOwned, vec::Vec, string::String};
use lazy_static::lazy_static;

use kampela_system::{
    devices::{psram::{psram_read_at_address, CheckedMetadataMetal, ExternalPsram, PsramAccess},
    se_aes_gcm::{self, Out},
    se_rng,
    touch::{Read, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}},
    draw::FrameBuffer,
    if_in_free,
    in_free,
    parallel::Operation
};
use substrate_parser::{decode_as_call_unmarked, decode_extensions_unmarked, ShortSpecs, TransactionUnmarkedParsed};
use kampela_system::devices::flash::*;
use crate::nfc::NfcTransactionPsramAccess;
use kampela_ui::{display_def::*, platform::{public_from_entropy, PinCode, Platform}, uistate};
use embedded_graphics::prelude::Point;

use primitive_types::H256;

use schnorrkel::{
    context::attach_rng,
    signing_context,
};

const SIGNING_CTX: &[u8] = b"substrate";

pub struct HALHandle {
    pub rng: se_rng::SeRng,
}

impl HALHandle {
    pub fn new() -> Self {
        let rng = se_rng::SeRng{};
        Self {
            rng: rng,
        }
    }
}

/// UI handler
pub struct UI {
    pub state: uistate::UIState<Hardware>,
    status: UIStatus,
    update: uistate::UpdateRequest,
}

impl UI {
    /// Start of UI.
    pub fn init() -> Self {
        let mut update = uistate::UpdateRequest::new();
        update.set_slow();
        let hardware = Hardware::new();
        let mut h = HALHandle::new();
        let state = uistate::UIState::new(hardware, &mut h);
        return Self {
            state: state,
            status: UIStatus::Listen,
            update: update,
        }
    }

    /// Call in event loop to progress through UI state
    pub fn advance(&mut self, voltage: i32) -> Option<bool> {
        match self.status {
            UIStatus::Listen => {
                self.listen();
                Some(true)
            },
            UIStatus::DisplayOperation => {
                match self.state.display().advance(voltage) {
                    Some(c) => {
                        if c {
                            self.status = UIStatus::Listen;
                        }
                        Some(false)
                    },
                    None => None,
                }
            },
            UIStatus::TouchOperation(ref mut touch) => {
                match touch.advance(()) {
                    Ok(Some(touch)) => {
                        if let Some(point) = convert(touch) {
                            let mut h = HALHandle::new();
                            self.update = self.state.handle_tap::<FrameBuffer>(point, &mut h).unwrap();
                            self.status = UIStatus::Listen;
                        };
                        Some(false)
                    },
                    Ok(None) => {None},
                    Err(e) => panic!("{:?}", e),
                }
            },
        }
    }

    fn listen(&mut self) {
        // 1. update ui if needed
        let f = self.update.read_fast();
        let s = self.update.read_slow();
        let p = self.update.read_part();
        let i = self.update.read_hidden();
        
        if i || f || s || p.is_some() {
            let mut h = HALHandle::new();
            self.update = self.state.render::<FrameBuffer>(f || s, &mut h).expect("guaranteed to work, no errors implemented");

            if !i {
                self.status = UIStatus::DisplayOperation;

                if f {
                    self.state.display().request_fast();
                }
                if s {
                    self.state.display().request_full();
                }
                if let Some(a) = p {
                    self.state.display().request_part(a);
                }
            }
            return;
        }

        // 2. read input if possible
        if if_in_free(|peripherals|
            peripherals.GPIO_S.if_.read().extif0().bit_is_set()
        ).unwrap() && matches!(self.status, UIStatus::Listen) {
            self.status = UIStatus::TouchOperation(Read::new(()));
        };
    }

    pub fn handle_message(&mut self, message: String) {
        let mut h = HALHandle::new();
        self.update = self.state.handle_message(message, &mut h);
    }

    pub fn handle_transaction(&mut self, transaction: NfcTransactionPsramAccess) {
        let mut h = HALHandle::new();
        self.state.platform.set_transaction(transaction);
        self.update = self.state.handle_transaction(&mut h);
    }

    pub fn handle_address(&mut self, addr: [u8; 76]) {
        self.update = self.state.handle_address(addr);
    }
}

/// General status of UI
///
/// There is no sense in reading input while screen processes last event, nor refreshing the screen
/// before touch was parsed
enum UIStatus {
    /// Event listening state, default
    Listen,
    /// Screen update started
    DisplayOperation,
    /// Touch event processing
    TouchOperation(Read<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>),
}

pub struct Hardware {
    pin: PinCode,
    pub protected: Option<[u8; 1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN]>,
    pub public_key: Option<[u8; 32]>,
    display: FrameBuffer,
    address: Option<[u8; 76]>,
    transaction_psram_access: Option<NfcTransactionPsramAccess>,
}

impl Hardware {
    pub fn new() -> Self {
        let protected = None;
        let public_key = None;
        let pin_set = false; // TODO query storage
        let mut h = HALHandle::new();
        let pin = [0; 4];
        let display = FrameBuffer::new_white();
        Self {
            pin: pin,
            protected: protected,
            public_key: public_key,
            display: display,
            address: None,
            transaction_psram_access: None,
        }
    }
}

impl Platform for Hardware {
    type HAL = HALHandle;
    type Rng = se_rng::SeRng;
    type Display = FrameBuffer;
    type NfcTransaction = NfcTransactionPsramAccess;

    fn rng<'b>(h: &'b mut HALHandle) -> &'b mut Self::Rng {
        &mut h.rng
    }

    fn pin(&self) -> &PinCode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut PinCode {
        &mut self.pin
    }

    fn display(&mut self) -> &mut <Self as Platform>::Display {
        &mut self.display
    }

    fn store_entropy(&mut self, e: &[u8]) {
        let mut protected = [0u8; 1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN];
        let public_key = public_from_entropy(e);
        let len = e.len();
        // encoding entropy
        in_free(|peripherals| {
            let out = if len != 0 {
                se_aes_gcm::create_key(peripherals).unwrap();
                se_aes_gcm::aes_gcm_encrypt(
                    peripherals,
                    [0; se_aes_gcm::AAD_LEN],
                    [0; se_aes_gcm::IV_LEN],
                    e.to_vec(),
                ).unwrap()
            } else {
                Out{
                    data: [0u8; se_aes_gcm::SECRET_MAX_LEN],
                    len: 0,
                    tag: [0; se_aes_gcm::TAG_LEN]
                }
            };

            protected[0] = out.len as u8;
            protected[1..1+se_aes_gcm::SECRET_MAX_LEN].copy_from_slice(&out.data);
            protected[1+se_aes_gcm::SECRET_MAX_LEN..1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN].copy_from_slice(&out.tag);
            protected[1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN..].copy_from_slice( unsafe { &se_aes_gcm::KEY_BUFFER });
        });

        // stroring encoded entropy and publilc key
        let mut payload = [0u8; 1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN + 32];
        payload[0..1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN].copy_from_slice(&protected);
        payload[1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN..].copy_from_slice(&public_key.unwrap_or([0u8; 32]));
        
        let mut data = [0u8; 2+1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN + 32];
        in_free(|peripherals| {
            flash_wakeup(peripherals);

            flash_unlock(peripherals);
            flash_erase_page(peripherals, 0);
            flash_wait_ready(peripherals);

            flash_unlock(peripherals);

            flash_write_page(peripherals, 0, &payload);
            flash_wait_ready(peripherals);

            flash_read(peripherals, 0, &mut data);
        });

        // Incorrect behavior of flash after wakeup. It reads two bytes of zeroes before the actual data stored in flash.
        if &data[2..] != &payload {
            panic!("Failed to save seedphrase: {:?} ||| {:?}", &data[2..35], &payload[..33]);
        }
        
        self.public_key = public_key;
        self.protected = if len != 0 {
            Some(protected)
        } else {
            None
        }
    }

    fn read_entropy(&mut self) {
        let mut data = [0u8; 1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN + 32];
        in_free(|peripherals| {
            // Make sure that flash is ok
            flash_wakeup(peripherals);
            flash_wait_ready(peripherals);
            let fl_id = flash_get_id(peripherals);
            let fl_len = flash_get_size(peripherals);
            if (fl_id == 0) || (fl_len == 0) {
                panic!("Flash error");
            }
            flash_read(peripherals, 0, &mut data);
            flash_sleep(peripherals);
        });
        match data[0] {
            0 => {
                self.protected = None;
                self.public_key = None;
            },
            16 | 20 | 24 | 28 | 32 => {
                self.protected = Some(
                    data[0..1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN]
                        .try_into()
                        .expect("static length")
                );
                self.public_key = Some(
                    data[1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN+se_aes_gcm::KEY_BUFFER_LEN..]
                        .try_into()
                        .expect("static length")
                );
            },
            255 => {
                self.protected = None;
                self.public_key = None;
            },
            _ => {
                self.store_entropy(&Vec::new());
                panic!("Seed storage corrupted! Wiping seed...");
            },
        }
    }

    fn entropy(&self) -> Option<Vec<u8>> {
        let mut entropy = None;
        if let Some(p) = self.protected {

            let recovered_out = se_aes_gcm::Out {
                data: p[1..1+se_aes_gcm::SECRET_MAX_LEN].try_into().expect("static length"),
                len: p[0] as usize,
                tag: p[1+se_aes_gcm::SECRET_MAX_LEN..1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN].try_into().expect("static length"),
            };
            unsafe { se_aes_gcm::KEY_BUFFER = p[1+se_aes_gcm::SECRET_MAX_LEN+se_aes_gcm::TAG_LEN..].try_into().expect("static length"); }
    
            if recovered_out.len != 0 {
                in_free(|peripherals| {
                    let out = se_aes_gcm::aes_gcm_decrypt(
                        peripherals,
                        &recovered_out,
                        [0u8; se_aes_gcm::AAD_LEN],
                        [0u8; se_aes_gcm::IV_LEN],
                    ).unwrap();
                    entropy = Some(out.data[..out.len].to_vec());
                });
            }
        }
        entropy
    }

    fn public(&self) -> Option<[u8; 32]> {
        self.public_key
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_transaction(&mut self, transaction: Self::NfcTransaction) {
        self.transaction_psram_access = Some(transaction);
    }


    fn call(&mut self) -> Option<(String, &mut <Self as Platform>::Display)> {
        let transaction_psram_access = match self.transaction_psram_access {
            Some(ref a) => a,
            None => return None
        };
        
        let call_data = read_from_psram(&transaction_psram_access.call_to_sign_psram_access);
    
        let (
            checked_metadata_metal,
            specs,
            spec_name
        ) = read_checked_metadata_metal(&transaction_psram_access.metadata_psram_access);
    
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
        let decoded_call = decoded_call_option.unwrap();

        let carded = decoded_call.card(0, &specs, &spec_name);
        let call = carded
            .into_iter()
            .map(|card| card.show())
            .collect::<Vec<String>>()
            .join("\n");

        Some((call, &mut self.display))
    }

    fn extensions(&mut self) -> Option<(String, &mut <Self as Platform>::Display)> {
        let transaction_psram_access = match self.transaction_psram_access {
            Some(ref a) => a,
            None => return None
        };
        
        let extension_data = read_from_psram(&transaction_psram_access.extension_to_sign_psram_access);
    
        let (
            checked_metadata_metal,
            specs,
            spec_name
        ) = read_checked_metadata_metal(&transaction_psram_access.metadata_psram_access);

        let genesis_hash = H256(
            read_from_psram(&transaction_psram_access.genesis_hash_bytes_psram_access)
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
                genesis_hash
            ).unwrap();
    
            decoded_extension_option = Some(decoded_extension);
        });
        let decoded_extension = decoded_extension_option.unwrap();

        let mut carded = Vec::new();
        for ext in decoded_extension.iter() {
            let addition_set = ext.card(0, true, &specs, &spec_name);
            if !addition_set.is_empty() {
                carded.extend_from_slice(&addition_set)
            }
        }
        let extensions = carded
            .into_iter()
            .map(|card| card.show())
            .collect::<Vec<String>>()
            .join("\n");

        Some((extensions, &mut self.display))
    }

    fn signature(&mut self, h: &mut Self::HAL) -> ([u8; 130], &mut <Self as Platform>::Display) {
        let transaction_psram_access = match self.transaction_psram_access {
            Some(ref a) => a,
            None => panic!("qr generation failed")
        };
        
        let data_to_sign_psram_access = PsramAccess {
            start_address: transaction_psram_access.call_to_sign_psram_access.start_address,
            total_len:
                transaction_psram_access.call_to_sign_psram_access.total_len
                + &transaction_psram_access.extension_to_sign_psram_access.total_len
        };
        let data_to_sign = read_from_psram(&data_to_sign_psram_access);

        let context = signing_context(SIGNING_CTX);
        let signature = self.pair()
            .unwrap()
            .sign(attach_rng(context.bytes(&data_to_sign), Self::rng(h)));

        let mut signature_with_id: [u8; 65] = [1; 65];
        signature_with_id[1..].copy_from_slice(&signature.to_bytes());
        let signature_with_id_bytes = hex::encode(signature_with_id)
            .into_bytes()
            .try_into()
            .expect("static length");

        (signature_with_id_bytes, &mut self.display)
    }

    fn address(&mut self) -> (&[u8; 76], &mut <Self as Platform>::Display) {
        if let Some(ref a) = self.address {
            (a, &mut self.display)
        } else {
            panic!("qr generation failed");
        }
    }

}

lazy_static! {
    // MAGIC calibration numbers obtained through KOLIBRI tool
    static ref AFFINE_MATRIX: Affine2<f32> = Affine2::from_matrix_unchecked(
        OMatrix::from_rows(&[
            RowVector3::<f32>::new(1.0022, -0.0216, -4.2725),
            RowVector3::<f32>::new(0.0061, 1.1433, -13.7305),
            RowVector3::<f32>::new(0.0, 0.0, 1.0),
        ])
    );
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

fn read_from_psram(psram_access: &PsramAccess) -> Vec<u8> {
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

pub fn convert(touch_data: [u8; LEN_NUM_TOUCHES]) -> Option<Point> {
    if touch_data[0] == 1 {
        let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
        let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
        let touch = Point::new(SCREEN_SIZE_X as i32 - detected_x, detected_y);

        let touch_as_point2 = Point2::new(touch.x as f32, touch.y as f32);
        let display_as_point2 = AFFINE_MATRIX.transform_point(&touch_as_point2);

        Some(
            Point {
                x: display_as_point2.coords[0] as i32,
                y: display_as_point2.coords[1] as i32,
            }
        )
    } else { None }
}


