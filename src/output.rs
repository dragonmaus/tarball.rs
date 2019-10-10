use bzip2::write::BzEncoder;
use flate2::write::GzEncoder;
use std::{
    fs::File,
    io::{self, Write},
};
use xz2::write::XzEncoder;

use crate::{Compression, Compressor};

pub enum Output {
    File(File),
    GZip(Box<GzEncoder<File>>),
    BZip(Box<BzEncoder<File>>),
    XZip(Box<XzEncoder<File>>),
}

impl Output {
    pub fn new(file: File, compressor: &Compressor, level: Compression) -> Output {
        match compressor {
            Compressor::None => Output::File(file),
            Compressor::GZip => Output::GZip(Box::new(GzEncoder::new(file, level.into()))),
            Compressor::BZip => Output::BZip(Box::new(BzEncoder::new(file, level.into()))),
            Compressor::XZip => Output::XZip(Box::new(XzEncoder::new(file, level.into()))),
        }
    }

    pub fn finish(&mut self) -> io::Result<()> {
        match self {
            Output::File(_) => (),
            Output::GZip(inner) => inner.try_finish()?,
            Output::BZip(inner) => inner.try_finish()?,
            Output::XZip(inner) => inner.try_finish()?,
        }

        Ok(())
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::File(inner) => inner.write(buf),
            Output::GZip(inner) => inner.write(buf),
            Output::BZip(inner) => inner.write(buf),
            Output::XZip(inner) => inner.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::File(inner) => inner.flush(),
            Output::GZip(inner) => inner.flush(),
            Output::BZip(inner) => inner.flush(),
            Output::XZip(inner) => inner.flush(),
        }
    }
}
