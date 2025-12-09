// === Imports ===
use std::fmt;
use std::io;

// === Types ===
#[derive(Debug)]
pub enum ErebusError {

    Invalid,
    EmptyTable,
    EmptyColumn,
    EmptyVector,
    EmptySelection,

    VectorLengthMismatch { expected: usize, found: usize },
    ColumnLengthMismatch { expected: usize, found: usize },
    RowLengthMismatch { expected: usize, found: usize },

    ColumnNotFound(String),
    DuplicateSelection(String),

    TypeMismatch { expected: String, found: String },

    IndexOutOfBounds(usize),
    InvalidOperation(String),
    InvalidDtype(String),
    JoinKeyMismatch,
    ColumnDataMismatch,

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
    OOB(String),
    // … add others as library grows
}

// === Impl ===
impl fmt::Display for ErebusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid operation"),
            Self::EmptyVector => write!(f, "The vector is empty"),
            Self::EmptyTable => write!(f, "The table is empty"),
            Self::EmptyColumn => write!(f, "The column is empty"),
            Self::EmptySelection => write!(f, "No columns selected"),
            Self::VectorLengthMismatch { expected, found } =>
                write!(f, "Vector length mismatch: expected {}, found {}", expected, found),
            Self::ColumnLengthMismatch { expected, found } =>
                write!(f, "Column length mismatch: expected {}, found {}", expected, found),
            Self::RowLengthMismatch { expected, found } =>
                write!(f, "Row length mismatch: expected {}, found {}", expected, found),
            Self::ColumnNotFound(name) =>
                write!(f, "Column not found: {}", name),
            Self::DuplicateSelection(name) =>
                write!(f, "Duplicate column selection: {}", name),
            Self::TypeMismatch { expected, found } =>
                write!(f, "Type mismatch: expected {}, found {}", expected, found),
            Self::IndexOutOfBounds(idx) =>
                write!(f, "Index out of bounds: {}", idx),
            Self::InvalidOperation(op) =>
                write!(f, "Invalid operation: {}", op),
            Self::InvalidDtype(s) =>
                write!(f, "Invalid dtype: {}", s),
            Self::JoinKeyMismatch =>
                write!(f, "Join key mismatch"),
            Self::ColumnDataMismatch =>
                write!(f, "Column names count and ColumnData count don't match"),
            Self::IoError(e) =>
                write!(f, "I/O error: {}", e),
            Self::InvalidMagic =>
                write!(f, "Invalid file magic — not a .erebus file"),
            Self::InvalidHeader =>
                write!(f, "Invalid file header"),
            Self::InvalidVersion { expected, found } =>
                write!(f, "Version mismatch: expected {}, found {}", expected, found),
            Self::UnknownObjectType(v) =>
                write!(f, "Unknown object type code: {}", v),
            Self::UnknownBaseType(v) =>
                write!(f, "Unknown base type code: {}", v),
            Self::UnknownEncodingType(v) =>
                write!(f, "Unknown encoding type code: {}", v),
            Self::UnexpectedEof =>
                write!(f, "Unexpected end of file"),
            Self::StreamLengthMismatch { expected, found } =>
                write!(f, "Stream length mismatch: expected {}, found {}", expected, found),
            Self::EncodeError(msg) =>
                write!(f, "Encoding error: {}", msg),
            Self::DecodeError(msg) =>
                write!(f, "Decoding error: {}", msg),
            Self::OOB(msg) => write!(f, "{}", msg),
        }
    }
}

// Allow using `?` with this error
impl std::error::Error for ErebusError {}

impl From<io::Error> for ErebusError {
    fn from(e: io::Error) -> Self {
        ErebusError::IoError(e)
    }
}