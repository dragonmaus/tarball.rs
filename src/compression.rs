#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Compression(u32);

impl Compression {
    pub fn new(level: u32) -> Compression {
        Compression(level)
    }

    pub fn none() -> Compression {
        Compression(0)
    }

    pub fn fast() -> Compression {
        Compression(1)
    }

    pub fn best() -> Compression {
        Compression(9)
    }

    pub fn level(self) -> u32 {
        self.0
    }
}

impl Default for Compression {
    fn default() -> Compression {
        Compression(6)
    }
}

impl Into<u32> for Compression {
    fn into(self) -> u32 {
        self.level()
    }
}

impl Into<bzip2::Compression> for Compression {
    fn into(self) -> bzip2::Compression {
        bzip2::Compression::new(self.level())
    }
}

impl Into<flate2::Compression> for Compression {
    fn into(self) -> flate2::Compression {
        flate2::Compression::new(self.level())
    }
}
