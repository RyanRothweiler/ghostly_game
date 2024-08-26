use std::ffi::*;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    MissingFile(String),
    ShaderCompilation(String),
    ShaderProgramLink(String),
    FFIStringConvert,
    Utf8Error,
    IOError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ObjTokenParsingError,
    CreateVAO,

    // windows plaform errors
    WindowsGetFileAttributes,
    WindowCopyFile,
    WindowsDeleteFile,
    WindowsLoadLibrary,

    // webg lerrors
    WebGlMissingVAO,
    WebGlCreateBuffer,
}

impl From<FromBytesUntilNulError> for Error {
    fn from(_error: FromBytesUntilNulError) -> Self {
        Error::FFIStringConvert
    }
}

impl From<Utf8Error> for Error {
    fn from(_error: Utf8Error) -> Self {
        Error::Utf8Error
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(error)
    }
}
