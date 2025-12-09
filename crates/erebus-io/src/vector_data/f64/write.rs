// === Imports ===
use crate::prelude::*;
use std::io::{Write, Seek};

// === Impl ===

pub struct F64Writer<'a, W: Write + Seek> {
    pub(crate) writer: &'a mut ErebusWriter<W>,
}

impl<'a, W: Write + Seek> F64Writer<'a, W> {
    pub fn factored(self) -> FactoredWriter<'a, W> {
        FactoredWriter { writer: self.writer }
    }

    pub fn raw(self) -> RawWriter<'a, W> {
        RawWriter { writer: self.writer }
    }
}