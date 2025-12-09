// === Imports ===
use crate::prelude::*;
use std::io::{Read, Seek};

// === Impl ===

pub struct F64FactoredReadStreams {
    pub validity_bytes: Vec<u8>,
    pub tags: Vec<u8>,
    pub sign_bytes: Vec<u8>,
    pub exp_ddelta: Vec<i16>,
    pub mant_xor: Vec<u64>,
}

#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn read_f64_factored_streams<R: Read + Seek>(
    reader: &mut ErebusReader<R>,
    h: &F64FactoredHeader,
    compression: CompressionType,
) -> Result<F64FactoredReadStreams, ErebusError>
{
    // For bytes we can either read raw (None) or via stream helper (compressed)
    let validity_bytes = match compression {
        CompressionType::None => reader.read_validity(h.validity_len as usize)?,
        _ => reader.read_stream_bytes(compression)?,
    };

    let tags = match compression {
        CompressionType::None => reader.read_bytes(h.tags_len as usize)?,
        _ => reader.read_stream_bytes(compression)?,
    };

    let sign_bytes = match compression {
        CompressionType::None => reader.read_bytes(h.sign_len as usize)?,
        _ => reader.read_stream_bytes(compression)?,
    };

    // For typed streams we always go through the compression-aware helpers.
    let exp_ddelta = reader.read_stream_i16(h.exp_len as usize, compression)?;
    let mant_xor   = reader.read_stream_u64(h.mant_len as usize, compression)?;

    Ok(F64FactoredReadStreams {
        validity_bytes,
        tags,
        sign_bytes,
        exp_ddelta,
        mant_xor,
    })
}

#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn decode_f64_factored(
    s: &F64FactoredReadStreams,
    n_rows: usize,
) -> (Vec<f64>, BitVec)
{
    let mut out_vals  = Vec::with_capacity(n_rows);
    let mut out_valid = BitVec::with_capacity(n_rows);

    let mut sign_byte = 0usize;
    let mut sign_bit  = 0usize;

    let mut next_sign = |s: &F64FactoredReadStreams,
                         sb: &mut usize,
                         bi: &mut usize| -> u64 {
        let b = s.sign_bytes[*sb];
        let bit = ((b >> *bi) & 1) as u64;

        *bi += 1;
        if *bi == 8 {
            *bi = 0;
            *sb += 1;
        }
        bit
    };

    let mut exp_i   = 0usize;
    let mut mant_i  = 0usize;

    let mut prev_exp:  Option<i16> = None;
    let mut prev_d1:   Option<i16> = None;
    let mut prev_mant: Option<u64> = None;

    for row in 0..n_rows {
        let vb = s.validity_bytes[row / 8];
        let is_valid = ((vb >> (row % 8)) & 1) == 1;
        out_valid.push(is_valid);

        if !is_valid {
            out_vals.push(f64::default());
            continue;
        }

        let tag = s.tags[row];

        let v = match tag {
            0 => 0.0,

            1 => {
                // Normal
                let sign = next_sign(s, &mut sign_byte, &mut sign_bit) << 63;

                let d2 = s.exp_ddelta[exp_i];
                exp_i += 1;

                let d1 = prev_d1.map_or(d2, |pd1| d2 + pd1);
                prev_d1 = Some(d1);

                let exp = prev_exp.map_or(d1, |pe| pe + d1);
                prev_exp = Some(exp);

                let stored = s.mant_xor[mant_i];
                mant_i += 1;

                let mant = prev_mant.map_or(stored, |pm| pm ^ stored);
                prev_mant = Some(mant);

                let exp_bits = ((exp + 1023) as u64) << 52;
                f64::from_bits(sign | exp_bits | mant)
            }

            2 => {
                // Subnormal
                let sign = next_sign(s, &mut sign_byte, &mut sign_bit) << 63;

                let stored = s.mant_xor[mant_i];
                mant_i += 1;

                let mant = prev_mant.map_or(stored, |pm| pm ^ stored);
                prev_mant = Some(mant);

                f64::from_bits(sign | mant)
            }

            3 => {
                // +inf
                next_sign(s, &mut sign_byte, &mut sign_bit);
                f64::from_bits(0x7FFu64 << 52)
            }

            4 => {
                // -inf
                let sbit = next_sign(s, &mut sign_byte, &mut sign_bit);
                f64::from_bits((sbit << 63) | (0x7FFu64 << 52))
            }

            _ => panic!("Invalid f64 factored tag {}", tag),
        };

        out_vals.push(v);
    }

    (out_vals, out_valid)
}

#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn read_vectordata_f64_factored<R: Read + Seek>(
    reader: &mut ErebusReader<R>,
) -> Result<(Vec<f64>, BitVec), ErebusError>
{
    reader.read_magic_and_version()?;
    let g = reader.read_global_header()?;

    if g.object_type != ObjectType::VectorData {
        return Err(ErebusError::InvalidOperation("Expected VectorData".into()));
    }
    if g.base_type != BaseType::F64 {
        return Err(ErebusError::InvalidOperation("Expected F64".into()));
    }
    if g.encoding != EncodingType::F64Factored {
        return Err(ErebusError::InvalidOperation("Expected F64Factored".into()));
    }

    let compression = g.compression;

    let h = F64FactoredHeader::read(reader.inner_mut())?;
    let streams = read_f64_factored_streams(reader, &h, compression)?;
    Ok(decode_f64_factored(&streams, h.n_rows as usize))
}

pub struct FactoredReader<'a, R: Read + Seek> {
    pub(crate) reader: &'a mut ErebusReader<R>,
}

impl<'a, R: Read + Seek> FactoredReader<'a, R> {
    pub fn read(self) -> Result<(Vec<f64>, BitVec), ErebusError> {
        read_vectordata_f64_factored(self.reader)
    }
}