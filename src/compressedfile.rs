use std::{
    fs::File,
    io::{self, Write},
};

use crate::{Codec, Compression};

pub enum CompressedFile {
    File(File),
    Bzip2(bzip2::write::BzEncoder<File>),
    Deflate(flate2::write::DeflateEncoder<File>),
    Gzip(flate2::write::GzEncoder<File>),
    Lz4(lz4::Encoder<File>),
    Xz(xz2::write::XzEncoder<File>),
    Zstd(zstd::stream::write::Encoder<File>),
}

impl CompressedFile {
    pub fn create(name: &str, codec: Codec, level: Compression) -> io::Result<Self> {
        let file = File::create(name)?;
        Ok(match codec {
            Codec::None => Self::File(file),
            Codec::Bzip2 => Self::Bzip2(bzip2::write::BzEncoder::new(file, level.into())),
            Codec::Deflate => Self::Deflate(flate2::write::DeflateEncoder::new(file, level.into())),
            Codec::Gzip => Self::Gzip(flate2::write::GzEncoder::new(file, level.into())),
            Codec::Lz4 => Self::Lz4(
                lz4::EncoderBuilder::new()
                    .level(level.into_custom(1, 16, 0))
                    .build(file)?,
            ),
            Codec::Xz => Self::Xz(xz2::write::XzEncoder::new(file, level.into())),
            Codec::Zstd => Self::Zstd(zstd::stream::write::Encoder::new(
                file,
                level.into_custom_signed(1, 21, 0),
            )?),
        })
    }

    pub fn finish(self) -> io::Result<()> {
        match self {
            Self::File(_) => Ok(()),
            Self::Bzip2(mut writer) => writer.try_finish(),
            Self::Deflate(mut writer) => writer.try_finish(),
            Self::Gzip(mut writer) => writer.try_finish(),
            Self::Lz4(writer) => writer.finish().1,
            Self::Xz(mut writer) => writer.try_finish(),
            Self::Zstd(writer) => writer.finish().map(|_| ()),
        }
    }
}

impl Write for CompressedFile {
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::File(writer) => writer.flush(),
            Self::Bzip2(writer) => writer.flush(),
            Self::Deflate(writer) => writer.flush(),
            Self::Gzip(writer) => writer.flush(),
            Self::Lz4(writer) => writer.flush(),
            Self::Xz(writer) => writer.flush(),
            Self::Zstd(writer) => writer.flush(),
        }
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::File(writer) => writer.write(buf),
            Self::Bzip2(writer) => writer.write(buf),
            Self::Deflate(writer) => writer.write(buf),
            Self::Gzip(writer) => writer.write(buf),
            Self::Lz4(writer) => writer.write(buf),
            Self::Xz(writer) => writer.write(buf),
            Self::Zstd(writer) => writer.write(buf),
        }
    }
}
