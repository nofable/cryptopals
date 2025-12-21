use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptopalsError {
    #[error("xor length mismatch")]
    XORLengthMismatch,

    #[error("error converting fromUTF8")]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("invalid parameter")]
    InvalidParameter(String),

    #[error("from hex error")]
    FromHex(#[from] hex::FromHexError),
}
