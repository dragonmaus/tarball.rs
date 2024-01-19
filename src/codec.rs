#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Codec {
    #[default]
    None,
    Bzip2,
    Deflate,
    Gzip,
    Lz4,
    Xz,
    Zstd,
}

impl Codec {
    pub fn extension(self) -> String {
        match self {
            Codec::None => "",
            Codec::Bzip2 => ".bz2",
            Codec::Deflate => ".Z",
            Codec::Gzip => ".gz",
            Codec::Lz4 => ".lz4",
            Codec::Xz => ".xz",
            Codec::Zstd => ".zst",
        }
        .into()
    }
}
