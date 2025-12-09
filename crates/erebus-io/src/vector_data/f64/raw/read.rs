// === Imports ===
use crate::prelude::*;
use std::io::{Read, Seek};

// === Impl ===

#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn read_vectordata_f64_raw<R: Read + Seek>(
    reader: &mut ErebusReader<R>,
) -> Result<(Vec<f64>, BitVec), ErebusError>
{
    reader.read_magic_and_version()?;

    let global = reader.read_global_header()?;

    if global.object_type != ObjectType::VectorData {
        return Err(ErebusError::InvalidOperation(
            format!("Expected VectorData, got {:?}", global.object_type),
        ));
    }
    if global.base_type != BaseType::F64 {
        return Err(ErebusError::InvalidOperation(
            format!("Expected F64, got {:?}", global.base_type),
        ));
    }
    if global.encoding != EncodingType::F64Raw {
        return Err(ErebusError::InvalidOperation(
            format!("Expected F64Raw, got {:?}", global.encoding),
        ));
    }

    let raw_header = F64RawHeader::read(reader.inner_mut())?;

    let n_rows = raw_header.n_rows as usize;
    let validity_len = raw_header.validity_len as usize;

    let validity_bytes = reader.read_bytes(validity_len)?;
    let validity = unpack_validity_bitmap(&validity_bytes, n_rows);

    // count valid entries
    let valid_count = validity.iter().filter(|b| **b).count();
    let expected_raw_bytes = valid_count * 8;

    let raw_bytes = match global.compression {
        CompressionType::None => {
            reader.read_bytes(expected_raw_bytes)?
        }

        CompressionType::Zstd => {
            let compressed_len = reader.read_u64()? as usize;
            let compressed = reader.read_bytes(compressed_len)?;
            crate::compression::zstd_decompress(&compressed)?
        }

        CompressionType::Lz4 => {
            let compressed_len = reader.read_u64()? as usize;
            let compressed = reader.read_bytes(compressed_len)?;
            crate::compression::lz4_decompress(&compressed)?
        }
    };

    if raw_bytes.len() != expected_raw_bytes {
        return Err(ErebusError::InvalidOperation(format!(
            "Corrupt F64Raw payload: expected {} bytes, got {}",
            expected_raw_bytes,
            raw_bytes.len()
        )));
    }

    let mut valid_vals = Vec::with_capacity(valid_count);
    for chunk in raw_bytes.chunks_exact(8) {
        valid_vals.push(f64::from_le_bytes(chunk.try_into().unwrap()));
    }

    let mut out_vals = Vec::with_capacity(n_rows);
    let mut idx = 0;

    for i in 0..n_rows {
        if validity[i] {
            out_vals.push(valid_vals[idx]);
            idx += 1;
        } else {
            out_vals.push(f64::default());
        }
    }

    Ok((out_vals, validity))
}

pub struct RawReader<'a, R: Read + Seek> {
    pub(crate) reader: &'a mut ErebusReader<R>,
}

impl<'a, R: Read + Seek> RawReader<'a, R> {
    pub fn read(self) -> Result<(Vec<f64>, BitVec), ErebusError> {
        read_vectordata_f64_raw(self.reader)
    }
}