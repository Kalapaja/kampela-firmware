//! Operations with AES GCM keys by security element.

use alloc::vec::Vec;
use core::ptr::addr_of;

use efm32pg23_fix::Peripherals;

use crate::peripherals::se_command::{
    se_command_aes_gsm_decrypt, DataTransfer, RxError, SeCommand, SE_COMMAND_AES_GCM_ENCRYPT,
    SE_COMMAND_CREATE_KEY, SE_DATATRANSFER_REALIGN, SE_DATATRANSFER_STOP,
};
use crate::in_free;

pub const KEY_META_LEN: usize = 8;

pub const KEY_META: [u8; KEY_META_LEN] = [0; KEY_META_LEN];

pub const KEY_BUFFER_LEN: usize = 60;

pub static mut KEY_BUFFER: [u8; KEY_BUFFER_LEN] = [0; KEY_BUFFER_LEN];

pub const AAD_LEN: usize = 4;

pub const IV_LEN: usize = 12;

pub const SECRET_MAX_LEN: usize = 33;

pub const KEYSPEC: u32 = 0b00001001000000000000000000100000;

pub const TAG_LEN: usize = 16;

pub fn encode_entropy(e: &[u8]) -> [u8; 1 + SECRET_MAX_LEN + TAG_LEN + KEY_BUFFER_LEN] {
    let mut protected = [0u8; 1 + SECRET_MAX_LEN + TAG_LEN + KEY_BUFFER_LEN];

    let len = e.len();
    // encoding entropy
    in_free(|peripherals| {
        let out = if len != 0 {
            create_key(peripherals).unwrap();
            aes_gcm_encrypt(
                peripherals,
                [0; AAD_LEN],
                [0; IV_LEN],
                e.to_vec(),
            ).unwrap()
        } else {
            Out{
                data: [0u8; SECRET_MAX_LEN],
                len: 0,
                tag: [0; TAG_LEN]
            }
        };

        protected[0] = out.len as u8;
        protected[1..1+SECRET_MAX_LEN].copy_from_slice(&out.data);
        protected[1+SECRET_MAX_LEN..1+SECRET_MAX_LEN+TAG_LEN].copy_from_slice(&out.tag);
        protected[1+SECRET_MAX_LEN+TAG_LEN..].copy_from_slice( unsafe { &KEY_BUFFER });
    });

    protected
}

pub fn decode_entropy(protected: [u8; 1 + SECRET_MAX_LEN + TAG_LEN + KEY_BUFFER_LEN]) -> Vec<u8> {
    let recovered_out = Out {
        data: protected[1..1 + SECRET_MAX_LEN].try_into().expect("static length"),
        len: protected[0] as usize,
        tag: protected[1+SECRET_MAX_LEN..1 + SECRET_MAX_LEN + TAG_LEN].try_into().expect("static length"),
    };
    unsafe { KEY_BUFFER = protected[1 + SECRET_MAX_LEN + TAG_LEN..].try_into().expect("static length"); }

    let mut entropy = None;
    if recovered_out.len != 0 {
        in_free(|peripherals| {
            let out = aes_gcm_decrypt(
                peripherals,
                &recovered_out,
                [0u8; AAD_LEN],
                [0u8; IV_LEN],
            ).unwrap();
            entropy = Some(out.data[..out.len].to_vec());
        });
    }
    entropy.unwrap()
}

pub fn create_key(peripherals: &mut Peripherals) -> Result<(), RxError> {
    let command_word = SE_COMMAND_CREATE_KEY;

    let key_meta = KEY_META;

    let data_transfer_in0 = DataTransfer {
        data: addr_of!(key_meta[0]) as u32,
        next: SE_DATATRANSFER_STOP,
        length: KEY_META_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };

    let data_in = addr_of!(data_transfer_in0) as u32;

    let data_transfer_out0 = DataTransfer {
        data: unsafe { addr_of!(KEY_BUFFER[0]) as u32 },
        next: SE_DATATRANSFER_STOP,
        length: KEY_BUFFER_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };

    let data_out = addr_of!(data_transfer_out0) as u32;

    let parameters = [KEYSPEC];

    let se_command = SeCommand {
        command_word,
        data_in,
        data_out,
        parameters: parameters.as_slice(),
    };

    se_command.execute(peripherals)
}

pub fn aes_gcm_encrypt(
    peripherals: &mut Peripherals,
    aad: [u8; AAD_LEN],
    iv: [u8; IV_LEN],
    secret: Vec<u8>,
) -> Result<Out, RxError> {
    let command_word = SE_COMMAND_AES_GCM_ENCRYPT;

    let key_meta = KEY_META;

    let data_transfer_in4 = DataTransfer {
        data: addr_of!(secret[0]) as u32,
        next: SE_DATATRANSFER_STOP,
        length: secret.len() as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in3 = DataTransfer {
        data: addr_of!(aad[0]) as u32,
        next: addr_of!(data_transfer_in4) as u32,
        length: AAD_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in2 = DataTransfer {
        data: addr_of!(iv[0]) as u32,
        next: addr_of!(data_transfer_in3) as u32,
        length: IV_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in1 = DataTransfer {
        data: unsafe { addr_of!(KEY_BUFFER[0]) as u32 },
        next: addr_of!(data_transfer_in2) as u32,
        length: KEY_BUFFER_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in0 = DataTransfer {
        data: addr_of!(key_meta[0]) as u32,
        next: addr_of!(data_transfer_in1) as u32,
        length: KEY_META_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };

    let data_in = addr_of!(data_transfer_in0) as u32;

    let len = secret.len();

    // maximum length reserved for 32 byte entropy or 32 byte secret key and
    // 1 marker byte to distinguish the two
    let encoded: [u8; SECRET_MAX_LEN] = [0; SECRET_MAX_LEN];
    let tag: [u8; TAG_LEN] = [0; TAG_LEN];

    let data_transfer_out1 = DataTransfer {
        data: addr_of!(tag[0]) as u32,
        next: SE_DATATRANSFER_STOP,
        length: TAG_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_out0 = DataTransfer {
        data: addr_of!(encoded[0]) as u32,
        next: addr_of!(data_transfer_out1) as u32,
        length: len as u32 | SE_DATATRANSFER_REALIGN,
    };

    let data_out = addr_of!(data_transfer_out0) as u32;

    let parameters = [KEYSPEC, AAD_LEN as u32, len as u32];

    let se_command = SeCommand {
        command_word,
        data_in,
        data_out,
        parameters: parameters.as_slice(),
    };

    se_command.execute(peripherals)?;

    Ok(Out {
        data: encoded,
        len,
        tag,
    })
}

pub fn aes_gcm_decrypt(
    peripherals: &mut Peripherals,
    out_encoded: &Out,
    aad: [u8; AAD_LEN],
    iv: [u8; IV_LEN],
) -> Result<Out, RxError> {
    let encoded = out_encoded.data;
    let len = out_encoded.len;
    let tag = out_encoded.tag;
    let key_meta = KEY_META;

    let command_word = se_command_aes_gsm_decrypt(&tag);

    let data_transfer_in5 = DataTransfer {
        data: addr_of!(tag[0]) as u32,
        next: SE_DATATRANSFER_STOP,
        length: TAG_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in4 = DataTransfer {
        data: addr_of!(encoded[0]) as u32,
        next: addr_of!(data_transfer_in5) as u32,
        length: len as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in3 = DataTransfer {
        data: addr_of!(aad[0]) as u32,
        next: addr_of!(data_transfer_in4) as u32,
        length: AAD_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in2 = DataTransfer {
        data: addr_of!(iv[0]) as u32,
        next: addr_of!(data_transfer_in3) as u32,
        length: IV_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in1 = DataTransfer {
        data: unsafe { addr_of!(KEY_BUFFER[0]) as u32 },
        next: addr_of!(data_transfer_in2) as u32,
        length: KEY_BUFFER_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };
    let data_transfer_in0 = DataTransfer {
        data: addr_of!(key_meta[0]) as u32,
        next: addr_of!(data_transfer_in1) as u32,
        length: KEY_META_LEN as u32 | SE_DATATRANSFER_REALIGN,
    };

    let data_in = addr_of!(data_transfer_in0) as u32;

    let decoded: [u8; SECRET_MAX_LEN] = [0; SECRET_MAX_LEN];

    let data_transfer_out0 = DataTransfer {
        data: addr_of!(decoded[0]) as u32,
        next: SE_DATATRANSFER_STOP,
        length: len as u32 | SE_DATATRANSFER_REALIGN,
    };

    let data_out = addr_of!(data_transfer_out0) as u32;

    let parameters = [KEYSPEC, AAD_LEN as u32, len as u32];

    let se_command = SeCommand {
        command_word,
        data_in,
        data_out,
        parameters: parameters.as_slice(),
    };

    se_command.execute(peripherals)?;

    Ok(Out {
        data: decoded,
        len,
        tag,
    })
}

#[derive(Debug)]
pub struct Out {
    pub data: [u8; SECRET_MAX_LEN],
    pub len: usize,
    pub tag: [u8; TAG_LEN],
}
