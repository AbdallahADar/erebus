// === Imports ===
use crate::prelude::*;
use std::io::{Write, Seek};

// === Impl ===

// Top-level entrypoint: reader.vector_data()
pub struct VectorDataWriter<'a, W: Write + Seek> {
    pub(crate) writer: &'a mut ErebusWriter<W>,
}

impl<'a, W: Write + Seek> VectorDataWriter<'a, W> {
    pub fn f64(self) -> F64Writer<'a, W> {
        F64Writer { writer: self.writer }
    }
}