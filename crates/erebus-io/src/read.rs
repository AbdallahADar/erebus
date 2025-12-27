// === Imports ===
use crate::prelude::*;
use std::io::{Read, Seek, SeekFrom};

// === Impl ===

#[derive(Debug)]
pub struct ErebusReader<R: Read + Seek> {
    inner: R,
}

impl<R: Read + Seek> ErebusReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    pub fn inner_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    pub fn read_magic(&mut self) -> ErrorResult<()> {
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        if buf != EREBUS_MAGIC {
            return Err(ErebusError::InvalidMagic);
        }
        Ok(())
    }

    pub fn read_version(&mut self) -> ErrorResult<u8> {
        let mut buf = [0u8; 1];
        self.inner.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_magic_and_version(&mut self) -> ErrorResult<u8> {
        self.read_magic()?;
        self.read_version()
    }

    pub fn read_global_header(&mut self) -> ErrorResult<ErebusHeader> {
        // MUST match write_global_header()
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;

        let object_type = ObjectType::from_u8(buf[0])?;
        let base_type   = BaseType::from_u8(buf[1])?;
        let encoding    = EncodingType::from_u8(buf[2])?;
        let compression = CompressionType::from_u8(buf[3])?;

        Ok(ErebusHeader {
            object_type,
            base_type,
            encoding,
            compression,
        })
    }

    pub fn read_bytes(&mut self, len: usize) -> ErrorResult<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.inner.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_u64(&mut self) -> ErrorResult<u64> {
        let mut buf = [0u8; 8];
        self.inner.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn read_u64_stream(&mut self, len: usize) -> ErrorResult<Vec<u64>> {
        let bytes = self.read_bytes(len * 8)?;
        let mut out = Vec::with_capacity(len);
        for chunk in bytes.chunks_exact(8) {
            out.push(u64::from_le_bytes(chunk.try_into().unwrap()));
        }
        Ok(out)
    }

    pub fn read_i16_stream(&mut self, len: usize) -> ErrorResult<Vec<i16>> {
        let bytes = self.read_bytes(len * 2)?;
        let mut out = Vec::with_capacity(len);
        for chunk in bytes.chunks_exact(2) {
            out.push(i16::from_le_bytes([chunk[0], chunk[1]]));
        }
        Ok(out)
    }

    pub fn read_validity(&mut self, len: usize) -> ErrorResult<Vec<u8>> {
        self.read_bytes(len)
    }

    pub fn seek_abs(&mut self, offset: u64) -> ErrorResult<()> {
        self.inner.seek(SeekFrom::Start(offset))?;
        Ok(())
    }

    /// Reads a stream written via write_stream_bytes().
    /// If compressed, the function first reads a u64 block length,
    /// then decompresses the following `block_len` bytes.
    pub fn read_stream_bytes(
        &mut self,
        compression: CompressionType,
    ) -> ErrorResult<Vec<u8>> {
        match compression {
            CompressionType::None => {
                // Caller must know how many bytes to read.
                Err(ErebusError::InvalidOperation(
                    "read_stream_bytes(None) must not be used directly".into(),
                ))
            }

            CompressionType::Zstd => {
                // read length prefix
                let mut len_buf = [0u8; 8];
                self.inner.read_exact(&mut len_buf)?;
                let block_len = u64::from_le_bytes(len_buf) as usize;

                // read block
                let mut compressed = vec![0u8; block_len];
                self.inner.read_exact(&mut compressed)?;

                // decompress
                let decompressed = zstd::decode_all(&compressed[..])
                    .map_err(|e| ErebusError::IoError(std::io::Error::new(
                        std::io::ErrorKind::Other, e
                    )))?;

                Ok(decompressed)
            }

            CompressionType::Lz4 => {
                let mut len_buf = [0u8; 8];
                self.inner.read_exact(&mut len_buf)?;
                let block_len = u64::from_le_bytes(len_buf) as usize;

                let mut compressed = vec![0u8; block_len];
                self.inner.read_exact(&mut compressed)?;

                let decompressed =
                    lz4_flex::block::decompress_size_prepended(&compressed)
                        .map_err(|e| ErebusError::IoError(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("lz4 decompress error: {:?}", e)
                        )))?;

                Ok(decompressed)
            }
        }
    }

    pub fn read_stream_i16(
        &mut self,
        n: usize,
        compression: CompressionType,
    ) -> ErrorResult<Vec<i16>> {
        match compression {
            CompressionType::None => self.read_i16_stream(n),

            _ => {
                let bytes = self.read_stream_bytes(compression)?;
                if bytes.len() != n * 2 {
                    return Err(ErebusError::InvalidOperation(
                        format!("Compressed i16 stream wrong length: have {}, expected {}", bytes.len(), n * 2)
                    ));
                }

                let mut out = Vec::with_capacity(n);
                for chunk in bytes.chunks_exact(2) {
                    out.push(i16::from_le_bytes([chunk[0], chunk[1]]));
                }
                Ok(out)
            }
        }
    }

    pub fn read_stream_u64(
        &mut self,
        n: usize,
        compression: CompressionType,
    ) -> ErrorResult<Vec<u64>> {
        match compression {
            CompressionType::None => self.read_u64_stream(n),

            _ => {
                let bytes = self.read_stream_bytes(compression)?;
                if bytes.len() != n * 8 {
                    return Err(ErebusError::InvalidOperation(
                        format!(
                            "Compressed u64 stream wrong length: have {}, expected {}",
                            bytes.len(),
                            n * 8
                        )
                    ));
                }

                let mut out = Vec::with_capacity(n);
                for chunk in bytes.chunks_exact(8) {
                    out.push(u64::from_le_bytes(chunk.try_into().unwrap()));
                }
                Ok(out)
            }
        }
    }
}

impl<R: Read + Seek> ErebusReader<R> {
    pub fn vector_data(&mut self) -> VectorDataReader<'_, R> {
        VectorDataReader { reader: self }
    }
}