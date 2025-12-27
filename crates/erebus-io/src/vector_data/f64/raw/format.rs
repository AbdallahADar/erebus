// === Imports ===
use crate::prelude::*;
use std::fmt;
use std::io::{Read, Write};

// === Impl ===

/// EncodingType variant for raw f64
pub const ENCODING_F64_RAW: EncodingType = EncodingType::F64Raw;

/// Type-specific header for VectorData<f64> using RAW encoding.
///
/// Global header (ErebusHeader) is written/read separately and only tells us:
/// - ObjectType::VectorData
/// - BaseType::F64
/// - EncodingType::F64Raw
///
/// This header adds:
/// - n_rows:      logical row count
/// - validity_len: length in bytes of the validity bitmap
#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) struct F64RawHeader {
    pub n_rows: u64,
    pub validity_len: u64,
}

impl F64RawHeader {
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    pub(crate) fn new(n_rows: u64, validity_len: u64) -> Self {
        Self { n_rows, validity_len }
    }

    /// Write this raw header to the underlying writer (after the global header).
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    pub(crate) fn write<W: Write>(&self, w: &mut W) -> ErrorResult<()> {
        w.write_all(&self.n_rows.to_le_bytes())?;
        w.write_all(&self.validity_len.to_le_bytes())?;
        Ok(())
    }

    /// Read a raw header from the underlying reader (after the global header).
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    pub(crate) fn read<R: Read>(r: &mut R) -> ErrorResult<Self> {
        let mut buf = [0u8; 8];

        // n_rows
        r.read_exact(&mut buf)?;
        let n_rows = u64::from_le_bytes(buf);

        // validity_len
        r.read_exact(&mut buf)?;
        let validity_len = u64::from_le_bytes(buf);

        Ok(Self { n_rows, validity_len })
    }
}