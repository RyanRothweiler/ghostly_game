use std::ffi::*;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    MissingFile(String),
    ShaderCompilation(String),
    FFIStringConvert,
    Utf8Error,
}

impl From<FromBytesUntilNulError> for Error {
    fn from(error: FromBytesUntilNulError) -> Self {
        Error::FFIStringConvert
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8Error
    }
}
