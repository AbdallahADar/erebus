// === Imports ===
use crate::prelude::*;
use std::io::{Write, Seek};

// === Streams ===

pub struct F64FactoredStreams {
    pub validity_bytes: Vec<u8>,
    pub tags: Vec<u8>,
    pub sign_bytes: Vec<u8>,
    pub sign_count: usize,
    pub exp_ddelta: Vec<i16>,
    pub mant_xor: Vec<u64>,
}

impl F64FactoredStreams {
    pub fn new(n: usize) -> Self {
        Self {
            validity_bytes: Vec::with_capacity((n + 7) / 8),
            tags: Vec::with_capacity(n),
            sign_bytes: Vec::new(),
            sign_count: 0,
            exp_ddelta: Vec::new(),
            mant_xor: Vec::new(),
        }
    }
}

#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn encode_f64_factored_streams(
    values: &[f64],
    validity: &BitVec,
) -> F64FactoredStreams {
    let n = values.len();
    let mut s = F64FactoredStreams::new(n);

    // validity
    let mut vb = 0u8;
    let mut vb_i = 0u8;

    for i in 0..n {
        if validity[i] {
            vb |= 1 << vb_i;
        }
        vb_i += 1;

        if vb_i == 8 {
            s.validity_bytes.push(vb);
            vb = 0;
            vb_i = 0;
        }
    }
    if vb_i != 0 {
        s.validity_bytes.push(vb);
    }

    // previous state
    let mut prev_exp: Option<i16> = None;
    let mut prev_d1: Option<i16> = None;
    let mut prev_mant: Option<u64> = None;

    // sign packer
    let mut sb = 0u8;
    let mut sb_i = 0u8;

    let mut push_sign = |bitval: u8,
                         s: &mut F64FactoredStreams,
                         sb: &mut u8,
                         sb_i: &mut u8| {
        *sb |= (bitval & 1) << *sb_i;
        *sb_i += 1;
        s.sign_count += 1;
        if *sb_i == 8 {
            s.sign_bytes.push(*sb);
            *sb = 0;
            *sb_i = 0;
        }
    };

    // main loop
    for i in 0..n {
        if !validity[i] {
            s.tags.push(0);
            continue;
        }

        let bits = values[i].to_bits();
        let sign = ((bits >> 63) & 1) as u8;
        let exp_raw = ((bits >> 52) & 0x7FF) as u16;
        let mant = bits & 0x000F_FFFF_FFFF_FFFF;

        match exp_raw {
            0 => {
                if mant == 0 {
                    s.tags.push(0);
                } else {
                    s.tags.push(2);
                    push_sign(sign, &mut s, &mut sb, &mut sb_i);

                    let xor_m = prev_mant.map_or(mant, |pm| mant ^ pm);
                    prev_mant = Some(mant);
                    s.mant_xor.push(xor_m);
                }
            }

            0x7FF => {
                if mant != 0 {
                    panic!("NaN encountered in VectorData<f64>");
                }
                s.tags.push(if sign == 0 { 3 } else { 4 });
                push_sign(sign, &mut s, &mut sb, &mut sb_i);
            }

            _ => {
                s.tags.push(1);
                push_sign(sign, &mut s, &mut sb, &mut sb_i);

                let exp = (exp_raw as i16) - 1023;

                let d1 = prev_exp.map_or(exp, |pe| exp - pe);
                let d2 = prev_d1.map_or(d1, |pd1| d1 - pd1);
                s.exp_ddelta.push(d2);

                prev_exp = Some(exp);
                prev_d1 = Some(d1);

                let xor_m = prev_mant.map_or(mant, |pm| mant ^ pm);
                prev_mant = Some(mant);
                s.mant_xor.push(xor_m);
            }
        }
    }

    if sb_i != 0 {
        s.sign_bytes.push(sb);
    }

    s
}

#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn write_vectordata_f64_factored<W: Write + Seek>(
    writer: &mut ErebusWriter<W>,
    values: &[f64],
    validity: &BitVec,
) -> ErrorResult<()> {

    let s = encode_f64_factored_streams(values, validity);

    // MAGIC + header
    writer.write_magic_and_version()?;

    writer.write_global_header(&ErebusHeader::new(
        ObjectType::VectorData,
        BaseType::F64,
        EncodingType::F64Factored,
        writer.compression(),
    ))?;

    // size header
    F64FactoredHeader::new(
        values.len() as u64,
        s.validity_bytes.len() as u64,
        s.tags.len() as u64,
        s.sign_bytes.len() as u64,
        s.exp_ddelta.len() as u64,
        s.mant_xor.len() as u64,
    )
    .write(writer.inner_mut())?;

    writer.write_stream_bytes(&s.validity_bytes)?;
    writer.write_stream_bytes(&s.tags)?;
    writer.write_stream_bytes(&s.sign_bytes)?;
    writer.write_stream_i16(&s.exp_ddelta)?;
    writer.write_stream_u64(&s.mant_xor)?;

    Ok(())
}

// Public entry point for orchestrator

pub struct FactoredWriter<'a, W: Write + Seek> {
    pub(crate) writer: &'a mut ErebusWriter<W>,
}

impl<'a, W: Write + Seek> FactoredWriter<'a, W> {
    pub fn write(self, values: &[f64], validity: &BitVec) -> ErrorResult<()> {
        write_vectordata_f64_factored(self.writer, values, validity)
    }
}