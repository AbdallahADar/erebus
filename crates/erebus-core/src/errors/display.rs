// erebus-core/src/errors/display.rs

use std::fmt;
use super::ErebusError;

impl fmt::Display for ErebusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            // --- Common ---
            ErebusError::InvalidOperation(op) =>
                write!(f, "Invalid operation: {}", op),
            ErebusError::InvalidDtype(s) =>
                write!(f, "Invalid dtype: {}", s),
            ErebusError::TypeMismatch { expected, found } =>
                write!(f, "Type mismatch: expected {}, found {}", expected, found),
            ErebusError::LengthMismatch { expected, found } =>
                write!(f, "Length mismatch: expected {}, found {}", expected, found),
            ErebusError::IndexOutOfBounds { index, size } =>
                write!(f, "Index out of bounds: index {} not in 0..{}", index, size),

            // --- Vector ---
            ErebusError::EmptyVector =>
                write!(f, "The vector is empty"),
            ErebusError::JoinKeyMismatch =>
                write!(f, "Join key mismatch"),
            ErebusError::ColumnDataMismatch =>
                write!(f, "Column names count and ColumnData count don't match"),
            ErebusError::InvalidCutBins { reason } =>
                write!(f, "{}", reason),
            ErebusError::InvalidCutLabels { expected, found } =>
                write!(f, "Cut labels mismatch: expected {}, found {}", expected, found),

            // --- Time ---
            ErebusError::InvalidYear(y) =>
                write!(f, "Invalid year: {}", y),
            ErebusError::InvalidMonth(m) =>
                write!(f, "Invalid month: {} (expected 1..=12)", m),
            ErebusError::InvalidDay { year, month, day } =>
                write!(f, "Invalid day: {:04}-{:02}-{:02}", year, month, day),
            ErebusError::InvalidDate(s) =>
                write!(f, "Invalid date: {}", s),

            // --- IO ---
            ErebusError::IoError(e) =>
                write!(f, "I/O error: {}", e),
            ErebusError::InvalidMagic =>
                write!(f, "Invalid file magic — not a .erebus file"),
            ErebusError::InvalidHeader =>
                write!(f, "Invalid file header"),
            ErebusError::InvalidVersion { expected, found } =>
                write!(f, "Version mismatch: expected {}, found {}", expected, found),
            ErebusError::UnknownObjectType(v) =>
                write!(f, "Unknown object type code: {}", v),
            ErebusError::UnknownBaseType(v) =>
                write!(f, "Unknown base type code: {}", v),
            ErebusError::UnknownEncodingType(v) =>
                write!(f, "Unknown encoding type code: {}", v),
            ErebusError::UnexpectedEof =>
                write!(f, "Unexpected end of file"),
            ErebusError::StreamLengthMismatch { expected, found } =>
                write!(f, "Stream length mismatch: expected {}, found {}", expected, found),
            ErebusError::EncodeError(msg) =>
                write!(f, "Encoding error: {}", msg),
            ErebusError::DecodeError(msg) =>
                write!(f, "Decoding error: {}", msg),

        }
    }
}