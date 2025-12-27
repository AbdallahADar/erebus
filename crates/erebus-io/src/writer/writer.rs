// === Imports ===

use std::io::{Write, Seek};
use crate::prelude::*;

// === Impl ===

/// Core writer for the .erebus file format.
/// This struct does **not** know anything about specific types like VectorData.
/// It only knows how to write:
/// - magic bytes
/// - version
/// - header
/// - raw streams of bytes
/// Higher level functions (e.g., f64 factored writer) will call into this.
pub struct ErebusWriter<W: Write + Seek> {
    inner: W,
}

impl<W: Write + Seek> ErebusWriter<W> {

    /// Create a new ErebusWriter from any writable + seekable object.
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    /// Returns a mutable reference to the underlying writer.
    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Writes magic bytes + version. Always at the start of the file.
    pub fn write_magic_and_version(&mut self) -> ErrorResult<()> {
        self.inner.write_all(&EREBUS_MAGIC)?;
        self.inner.write_all(&[EREBUS_VERSION])?;
        Ok(())
    }

    /// Writes the `.erebus` header.
    pub fn write_header(&mut self, header: &ErebusHeader) -> ErrorResult<()> {
        // ObjectType
        self.inner.write_all(&[header.object_type.to_u8()])?;

        // BaseType
        self.inner.write_all(&[header.base_type.to_u8()])?;

        // EncodingType
        self.inner.write_all(&[header.encoding.to_u8()])?;

        // Row count
        self.inner.write_all(&header.n_rows.to_le_bytes())?;

        // Factored stream lengths
        self.inner.write_all(&header.tags_len.to_le_bytes())?;
        self.inner.write_all(&header.sign_len.to_le_bytes())?;
        self.inner.write_all(&header.exp_len.to_le_bytes())?;
        self.inner.write_all(&header.mant_len.to_le_bytes())?;
        self.inner.write_all(&header.quiet_len.to_le_bytes())?;
        self.inner.write_all(&header.nan_payload_len.to_le_bytes())?;

        Ok(())
    }

    /// Writes a raw byte slice (used by all column streams).
    pub fn write_stream_bytes(&mut self, buf: &[u8]) -> ErrorResult<()> {
        self.inner.write_all(buf)?;
        Ok(())
    }

    /// Writes a vector of u64 (mantissas, payloads, etc.)
    pub fn write_u64_stream(&mut self, values: &[u64]) -> ErrorResult<()> {
        for v in values {
            self.inner.write_all(&v.to_le_bytes())?;
        }
        Ok(())
    }

    /// Writes a vector of i16 (exponent deltas).
    pub fn write_i16_stream(&mut self, values: &[i16]) -> ErrorResult<()> {
        for v in values {
            self.inner.write_all(&v.to_le_bytes())?;
        }
        Ok(())
    }


    /// Future: writes bytes to file using a chosen compression codec.
    pub fn write_compressed_stream(
        &mut self,
        _buf: &[u8],
        _codec: crate::compression::Compression,
    ) -> ErrorResult<()> {
        unimplemented!("Compression will be implemented later");
    }
}