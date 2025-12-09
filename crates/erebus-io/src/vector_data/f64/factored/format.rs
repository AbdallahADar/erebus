// === Imports ===
use crate::prelude::*;
use std::fmt;
use std::io::{Read, Write};

// === Impl ===

/// Version for the f64 factored VectorData encoding.
/// This allows future evolution while keeping backward compatibility.
pub const F64_FACTORED_VERSION: u8 = 1;

/// Type-specific header for VectorData<F64> using the "factored" encoding.
///
/// Written immediately after the global ErebusHeader.
///
/// Layout after GLOBAL HEADER:
///
///    [factored_version: u8]
///    [n_rows: u64]
///    [validity_len: u64]     (bytes)
///    [tags_len: u64]         (rows)
///    [sign_len: u64]         (rows)
///    [exp_len: u64]          (#exp entries)
///    [mant_len: u64]         (#mantissa entries)
///
/// Streams then appear exactly in the order listed above.
///
/// NOTE:
/// - NULL entries are represented by validity bits = 0.
/// - Tags represent *only valid* entries:
///       0 = zero
///       1 = normal
///       2 = subnormal
///       3 = +inf
///       4 = -inf
#[derive(Debug, Clone)]
pub struct F64FactoredHeader {
    pub version: u8,

    pub n_rows: u64,

    // stream lengths:
    pub validity_len: u64,   // bytes
    pub tags_len: u64,       // number of tag entries (always == n_rows)
    pub sign_len: u64,       // number of signs written
    pub exp_len: u64,        // number of i16 exponents written
    pub mant_len: u64,       // number of u64 mantissas written
}

impl F64FactoredHeader {
    pub fn new(
        n_rows: u64,
        validity_len: u64,
        tags_len: u64,
        sign_len: u64,
        exp_len: u64,
        mant_len: u64,
    ) -> Self {
        Self {
            version: F64_FACTORED_VERSION,
            n_rows,
            validity_len,
            tags_len,
            sign_len,
            exp_len,
            mant_len,
        }
    }

    /// Write this header to the writer.
    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), ErebusError> {
        w.write_all(&[self.version])?;
        w.write_all(&self.n_rows.to_le_bytes())?;
        w.write_all(&self.validity_len.to_le_bytes())?;
        w.write_all(&self.tags_len.to_le_bytes())?;
        w.write_all(&self.sign_len.to_le_bytes())?;
        w.write_all(&self.exp_len.to_le_bytes())?;
        w.write_all(&self.mant_len.to_le_bytes())?;
        Ok(())
    }

    /// Read a factored header from the reader.
    pub fn read<R: Read>(r: &mut R) -> Result<Self, ErebusError> {
        let mut buf_u8 = [0u8; 1];
        let mut buf_u64 = [0u8; 8];

        r.read_exact(&mut buf_u8)?;
        let version = buf_u8[0];

        r.read_exact(&mut buf_u64)?;
        let n_rows = u64::from_le_bytes(buf_u64);

        r.read_exact(&mut buf_u64)?;
        let validity_len = u64::from_le_bytes(buf_u64);

        r.read_exact(&mut buf_u64)?;
        let tags_len = u64::from_le_bytes(buf_u64);

        r.read_exact(&mut buf_u64)?;
        let sign_len = u64::from_le_bytes(buf_u64);

        r.read_exact(&mut buf_u64)?;
        let exp_len = u64::from_le_bytes(buf_u64);

        r.read_exact(&mut buf_u64)?;
        let mant_len = u64::from_le_bytes(buf_u64);

        Ok(Self {
            version,
            n_rows,
            validity_len,
            tags_len,
            sign_len,
            exp_len,
            mant_len,
        })
    }
}