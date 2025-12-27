// === Imports ===
use crate::prelude::*;
use std::io::{Seek, Write};

// === Impl ===

#[derive(Debug)]
pub struct ErebusWriter<W: Write + Seek> {
    inner: W,
    compression: CompressionType,
}

impl<W: Write + Seek> ErebusWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            compression: CompressionType::None,
        }
    }

    pub fn set_compression(&mut self, c: CompressionType) {
        self.compression = c;
    }

    pub fn with_compression(mut self, c: CompressionType) -> Self {
        self.compression = c;
        self
    }

    pub fn compression(&self) -> CompressionType {
        self.compression
    }

    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    pub fn write_magic_and_version(&mut self) -> ErrorResult<()> {
        self.inner.write_all(&EREBUS_MAGIC)?;
        self.inner.write_all(&[EREBUS_VERSION])?;
        Ok(())
    }

    pub fn write_global_header(&mut self, header: &ErebusHeader) -> ErrorResult<()> {
        // MUST match ErebusReader::read_global_header
        self.inner.write_all(&[header.object_type.to_u8()])?;
        self.inner.write_all(&[header.base_type.to_u8()])?;
        self.inner.write_all(&[header.encoding.to_u8()])?;
        self.inner.write_all(&[header.compression.to_u8()])?;
        Ok(())
    }

    pub fn write_bytes(&mut self, buf: &[u8]) -> ErrorResult<()> {
        self.inner.write_all(buf)?;
        Ok(())
    }

    pub fn write_u64_stream(&mut self, values: &[u64]) -> ErrorResult<()> {
        for v in values {
            self.inner.write_all(&v.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn write_i16_stream(&mut self, values: &[i16]) -> ErrorResult<()> {
        for v in values {
            self.inner.write_all(&v.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn write_validity(&mut self, bitmap: &[u8]) -> ErrorResult<()> {
        self.inner.write_all(bitmap)?;
        Ok(())
    }


    /// Write an arbitrary byte stream, applying the writer's compression policy.
    /// For compressed modes, we prefix the compressed block with its length as u64.
    pub fn write_stream_bytes(&mut self, buf: &[u8]) -> ErrorResult<()> {
        match self.compression {
            CompressionType::None => self.write_bytes(buf),

            CompressionType::Zstd => {
                let compressed = zstd::encode_all(buf, 0)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                let len = compressed.len() as u64;
                self.inner.write_all(&len.to_le_bytes())?;
                self.inner.write_all(&compressed)?;
                Ok(())
            }

            CompressionType::Lz4 => {
                let compressed = lz4_flex::block::compress_prepend_size(buf);
                let len = compressed.len() as u64;
                self.inner.write_all(&len.to_le_bytes())?;
                self.inner.write_all(&compressed)?;
                Ok(())
            }
        }
    }

    pub fn write_stream_i16(&mut self, values: &[i16]) -> ErrorResult<()> {
        let mut buf = Vec::with_capacity(values.len() * 2);
        for v in values {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        self.write_stream_bytes(&buf)
    }

    pub fn write_stream_u64(&mut self, values: &[u64]) -> ErrorResult<()> {
        let mut buf = Vec::with_capacity(values.len() * 8);
        for v in values {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        self.write_stream_bytes(&buf)
    }
}

// Hook for the VectorData orchestrator
impl<W: Write + Seek> ErebusWriter<W> {
    pub fn vector_data(&mut self) -> VectorDataWriter<W> {
        VectorDataWriter { writer: self }
    }
}