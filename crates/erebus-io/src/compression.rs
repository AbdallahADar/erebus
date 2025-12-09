// === Imports ===
use crate::prelude::*;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};

// === Impl ===

pub fn zstd_compress(input: &[u8]) -> Result<Vec<u8>, ErebusError> {
    zstd::encode_all(input, 0)
        .map_err(|e| ErebusError::IoError(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("zstd compress error: {}", e)
            )
        ))
}

/// ZSTD decompression using the `zstd` crate.
/// Automatically detects the uncompressed size from the Zstd frame header.
pub fn zstd_decompress(input: &[u8]) -> Result<Vec<u8>, ErebusError> {
    zstd::decode_all(input)
        .map_err(|e| ErebusError::IoError(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("zstd decompress error: {}", e)
            )
        ))
}

/// LZ4 compression with size prefix.
/// Very fast, lower ratio than zstd.
pub fn lz4_compress(input: &[u8]) -> Result<Vec<u8>, ErebusError> {
    Ok(compress_prepend_size(input))
}

/// LZ4 decompression.
/// Expects lz4_flex's size-prepended format.
pub fn lz4_decompress(input: &[u8]) -> Result<Vec<u8>, ErebusError> {
    decompress_size_prepended(input)
        .map_err(|e| ErebusError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other,
                format!("lz4 decompress error: {}", e))
        ))
}