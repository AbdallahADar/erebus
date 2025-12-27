// === Imports ===
use std::fmt;
use std::io;

// === Impls ===

#[derive(Debug)]
pub enum ErebusError {

    // --- Common ---
    InvalidOperation(String),
    InvalidDtype(String),
    TypeMismatch { expected: String, found: String },

    LengthMismatch { expected: usize, found: usize },

    IndexOutOfBounds {
        index: usize,
        size: usize,
    },


    // --- Vector ---
    EmptyVector,
    JoinKeyMismatch,
    ColumnDataMismatch,
    InvalidCutBins { reason: String },
    InvalidCutLabels { expected: usize, found: usize },

    // --- Time ---
    InvalidYear(i32),
    InvalidMonth(u8),
    InvalidDay { year: i32, month: u8, day: u8 },
    InvalidDate(String),

    // --- IO ---
    IoError(io::Error),
    InvalidMagic,
    InvalidHeader,
    InvalidVersion { expected: u8, found: u8 },
    UnknownObjectType(u8),
    UnknownBaseType(u8),
    UnknownEncodingType(u8),
    UnexpectedEof,
    StreamLengthMismatch { expected: u64, found: u64 },
    EncodeError(String),
    DecodeError(String),

}

// Allows using `?` with this error
impl std::error::Error for ErebusError {}

impl From<io::Error> for ErebusError {
    fn from(e: io::Error) -> Self {
        ErebusError::IoError(e)
    }
}