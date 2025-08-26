use core::fmt;

pub enum ErrorTransaction {
    AddressUnmatch,
    SourceFingerprintUnmatch,
    SignReustDecodeError(EthSignRequestDecodeError)
}

impl fmt::Display for ErrorTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorTransaction::SignReustDecodeError(eth_sign_request_error) => {
                write!(f, "{}", eth_sign_request_error)?;
            },
            ErrorTransaction::AddressUnmatch => {
                write!(f, "Address does not match")?;
            },
            ErrorTransaction::SourceFingerprintUnmatch => {
                write!(f, "Source Fingerprint does not match")?;
            },
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum EthSignRequestDecodeError {
    UuidError(uuid::Error),
    UnknownSignDataType,
    InvalidDerivationPathString,
    NoEthSignRequestDataStored,
}

impl fmt::Display for EthSignRequestDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EthSignRequestDecodeError::UuidError(uuid_error) => {
                write!(f, "UUID parsing error: {}", uuid_error)?;
            },
            EthSignRequestDecodeError::UnknownSignDataType => {
                write!(f, "Unknown Sign Request Data type")?;
            },
            EthSignRequestDecodeError::InvalidDerivationPathString => {
                write!(f, "Invalid Derivation Path string")?;
            },
            EthSignRequestDecodeError::NoEthSignRequestDataStored => {
                write!(f, "No Sign Request Data stored")?;
            }
        };
        Ok(())
    }
}
