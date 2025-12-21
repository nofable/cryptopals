use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptopalsError {
    #[error("inputs must have the same length")]
    ParametersLengthMismatch,

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("invalid parameter")]
    InvalidParameter(String),

    #[error(transparent)]
    Hex(#[from] hex::FromHexError),

    #[error("Misshaped matrix in transpose")]
    MisshapedMatrix { reason: String },

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),
}
