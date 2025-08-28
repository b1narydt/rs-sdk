// Unified SDK error type
#[derive(Debug)]
pub enum SdkError {
    NotImplemented(&'static str),
    InvalidArgument(&'static str),
    ParseError(&'static str),
    CryptoError(&'static str),
    IoError,
    NetworkError,
}

pub type Result<T> = core::result::Result<T, SdkError>;
