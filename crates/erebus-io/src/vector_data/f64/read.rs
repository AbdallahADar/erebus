// === Imports ===
use crate::prelude::*;
use std::io::{Read, Seek};

// === Impl ===

pub struct F64Reader<'a, R: Read + Seek> {
    pub(crate) reader: &'a mut ErebusReader<R>,
}

impl<'a, R: Read + Seek> F64Reader<'a, R> {

    pub fn factored(self) -> FactoredReader<'a, R> {
        FactoredReader { reader: self.reader }
    }

    pub fn raw(self) -> RawReader<'a, R> {
        RawReader { reader: self.reader }
    }
}