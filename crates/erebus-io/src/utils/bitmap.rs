// === Imports ===
use crate::prelude::*;

// === Impl ===

/// Convert a BitVec (validity bitmap used by VectorData)
/// into packed bytes, LSB0 ordering.
///
/// Examples:
///   [true, false, true, true] → byte 0b00001101
///
#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn pack_validity_bitmap(valid: &BitVec) -> Vec<u8> {
    let n = valid.len();
    let mut out = Vec::with_capacity((n + 7) / 8);

    let mut byte: u8 = 0;
    let mut bit = 0;

    for i in 0..n {
        if valid[i] {
            byte |= 1 << bit;
        }
        bit += 1;

        if bit == 8 {
            out.push(byte);
            byte = 0;
            bit = 0;
        }
    }
    if bit != 0 {
        out.push(byte);
    }
    out
}

/// Convert packed bitmap bytes back into a BitVec.
/// n_rows tells us how many bits to expand.
#[cfg_attr(feature = "internal", visibility::make(pub))]
pub(crate) fn unpack_validity_bitmap(bytes: &[u8], n_rows: usize) -> BitVec {
    let mut bv = BitVec::with_capacity(n_rows);
    for i in 0..n_rows {
        let byte = bytes[i / 8];
        let bit = (byte >> (i % 8)) & 1;
        bv.push(bit == 1);
    }
    bv
}