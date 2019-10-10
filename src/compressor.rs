#[derive(Debug, Eq, PartialEq)]
pub enum Compressor {
    None,
    GZip,
    BZip,
    XZip,
}

impl Default for Compressor {
    fn default() -> Compressor {
        Compressor::None
    }
}
