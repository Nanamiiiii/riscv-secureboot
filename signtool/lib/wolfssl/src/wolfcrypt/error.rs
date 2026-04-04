#![allow(dead_code)]
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Errorkind {
    CMemError,
    KeybufferError,
    SigbufferError,
}

impl Errorkind {
    pub fn desc(&self) -> &'static str {
        match self {
            Errorkind::CMemError => "C ffi memory allocation error",
            Errorkind::KeybufferError => "Key buffer error",
            Errorkind::SigbufferError => "Signature buffer error",
        }
    }
}

pub struct Error {
    _error: _Error,
}

enum _Error {
    Simple(Errorkind),
    Native(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self._error {
            _Error::Simple(s) => f.write_str(s.desc()),
            _Error::Native(errno) => {
                f.write_fmt(format_args!("wolfCrypto returns error, {}", *errno))
            }
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self._error {
            _Error::Simple(_) => None,
            _Error::Native(_) => None,
        }
    }
}

impl From<Errorkind> for Error {
    fn from(value: Errorkind) -> Self {
        Error {
            _error: _Error::Simple(value),
        }
    }
}

impl Error {
    pub fn from_native(errno: i32) -> Self {
        Error {
            _error: _Error::Native(errno),
        }
    }
}
