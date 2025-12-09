// === Imports ===
use crate::prelude::*;
use std::fmt;

// === Impl ===

// Global .erebus file-level primitives.
// Every .erebus file begins with:
//   [MAGIC][VERSION][OBJECT_TYPE][BASE_TYPE][ENCODING_TYPE][COMPRESSION_TYPE]
// Then specialized headers follow depending on encoding.

/// 4-byte magic prefix for all .erebus files: "ERBS"
pub const EREBUS_MAGIC: [u8; 4] = *b"ERBS";

/// Global file format version.
/// This is the version tag at the universal level.
/// Each encoding may have its own sub-version.
pub const EREBUS_VERSION: u8 = 1;

// =============================================================
// High-Level Object Type
// =============================================================

/// What type of logical object the file stores.
/// This tells the reader how to interpret the specialized header that follows.
///
/// Vector       → no nulls, contiguous bytes
/// VectorData   → validity + typed payload (supports nulls)
/// Table        → multiple columns, each with its own internal encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjectType {
    Vector = 1,
    VectorData = 2,
    Table = 3,
}

impl ObjectType {
    pub fn from_u8(v: u8) -> Result<Self, ErebusError> {
        match v {
            1 => Ok(Self::Vector),
            2 => Ok(Self::VectorData),
            3 => Ok(Self::Table),
            _ => Err(ErebusError::InvalidDtype(format!("Invalid object type {}", v)))
        }
    }
    pub fn to_u8(self) -> u8 { self as u8 }
}

// =============================================================
// Base Primitive Type
// =============================================================

/// Logical scalar element stored in a Vector / VectorData / Table column.
/// Familiar to the user-level API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BaseType {
    F64 = 1,
    I64 = 2,
    Bool = 3,
    Text = 4,
    // Add more: F32, U64, Timestamp, etc.
}

impl BaseType {
    pub fn from_u8(v: u8) -> Result<Self, ErebusError> {
        match v {
            1 => Ok(Self::F64),
            2 => Ok(Self::I64),
            3 => Ok(Self::Bool),
            4 => Ok(Self::Text),
            _ => Err(ErebusError::InvalidDtype(format!("Invalid base type {}", v))),
        }
    }
    pub fn to_u8(self) -> u8 { self as u8 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EncodingType {
    F64Raw = 1,
    F64Factored = 2,
    // add others later...
}

impl EncodingType {
    pub fn from_u8(v: u8) -> Result<Self, ErebusError> {
        match v {
            1 => Ok(Self::F64Raw),
            2 => Ok(Self::F64Factored),
            _ => Err(ErebusError::InvalidDtype(format!("Invalid encoding {}", v))),
        }
    }
    pub fn to_u8(self) -> u8 { self as u8 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompressionType {
    None = 0,
    Zstd = 1,
    Lz4  = 2,
}

impl CompressionType {
    pub fn from_u8(v: u8) -> Result<Self, ErebusError> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Zstd),
            2 => Ok(Self::Lz4),
            _ => Err(ErebusError::InvalidDtype(format!("Invalid compression {}", v))),
        }
    }
    pub fn to_u8(self) -> u8 { self as u8 }
}

/// Universal file header written immediately after MAGIC + VERSION.
#[derive(Debug, Clone)]
pub struct ErebusHeader {
    pub object_type: ObjectType,
    pub base_type: BaseType,
    pub encoding: EncodingType,
    pub compression: CompressionType,
}

impl ErebusHeader {
    pub fn new(
        object_type: ObjectType,
        base_type: BaseType,
        encoding: EncodingType,
        compression: CompressionType,
    ) -> Self {
        Self { object_type, base_type, encoding, compression }
    }

    /// Write the 4-byte global header
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), ErebusError> {
        w.write_all(&[self.object_type.to_u8()])?;
        w.write_all(&[self.base_type.to_u8()])?;
        w.write_all(&[self.encoding.to_u8()])?;
        w.write_all(&[self.compression.to_u8()])?;
        Ok(())
    }

    /// Read the 4-byte global header
    pub fn read<R: std::io::Read>(r: &mut R) -> Result<Self, ErebusError> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;

        Ok(Self {
            object_type: ObjectType::from_u8(buf[0])?,
            base_type:   BaseType::from_u8(buf[1])?,
            encoding:    EncodingType::from_u8(buf[2])?,
            compression: CompressionType::from_u8(buf[3])?,
        })
    }
}