// === Imports ===
use crate::prelude::*;
use std::io::{Write, Seek};

// === Impl ===

#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn write_vectordata_f64_raw<W: Write + Seek>(
    writer: &mut ErebusWriter<W>,
    values: &[f64],
    validity: &BitVec,
) -> Result<(), ErebusError>
{
    assert_eq!(values.len(), validity.len());
    let n_rows = values.len() as u64;

    let validity_bytes = pack_validity_bitmap(validity);
    let validity_len = validity_bytes.len() as u64;

    writer.write_magic_and_version()?;
    writer.write_global_header(&ErebusHeader::new(
        ObjectType::VectorData,
        BaseType::F64,
        EncodingType::F64Raw,
        writer.compression(),
    ))?;

    let raw_header = F64RawHeader::new(n_rows, validity_len);
    raw_header.write(writer.inner_mut())?;

    writer.write_bytes(&validity_bytes)?;

    let mut buf = Vec::with_capacity(values.len() * 8);

    for i in 0..values.len() {
        if validity[i] {
            buf.extend_from_slice(&values[i].to_bits().to_le_bytes());
        }
    }

    writer.write_stream_bytes(&buf)?;

    Ok(())
}

// Wrapper struct
pub struct RawWriter<'a, W: Write + Seek> {
    pub(crate) writer: &'a mut ErebusWriter<W>,
}

impl<'a, W: Write + Seek> RawWriter<'a, W> {
    pub fn write(self, values: &[f64], validity: &BitVec) -> Result<(), ErebusError> {
        write_vectordata_f64_raw(self.writer, values, validity)
    }
}