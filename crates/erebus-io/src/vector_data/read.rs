// === Imports ===
use crate::prelude::*;
use std::io::{Read, Seek};

// === Impl ===

// Top-level entrypoint: reader.vector_data()
pub struct VectorDataReader<'a, R: Read + Seek> {
    pub(crate) reader: &'a mut ErebusReader<R>,
}

impl<'a, R: Read + Seek> VectorDataReader<'a, R> {
    pub fn f64(self) -> F64Reader<'a, R> {
        F64Reader { reader: self.reader }
    }
}