#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compression {
    Default,
    Maximum,
    Minimum,
}

impl Compression {
    pub fn best() -> Self {
        Compression::Maximum
    }

    pub fn fast() -> Self {
        Compression::Minimum
    }

    pub fn into_custom(self, min: u32, max: u32, def: u32) -> u32 {
        match self {
            Compression::Default => def,
            Compression::Maximum => max,
            Compression::Minimum => min,
        }
    }

    pub fn into_custom_signed(self, min: i32, max: i32, def: i32) -> i32 {
        match self {
            Compression::Default => def,
            Compression::Maximum => max,
            Compression::Minimum => min,
        }
    }
}

impl Default for Compression {
    fn default() -> Self {
        Compression::Default
    }
}

// Common values
impl From<Compression> for u32 {
    fn from(c: Compression) -> u32 {
        match c {
            Compression::Default => 6,
            Compression::Maximum => 9,
            Compression::Minimum => 1,
        }
    }
}

impl From<Compression> for bzip2::Compression {
    fn from(c: Compression) -> bzip2::Compression {
        match c {
            Compression::Default => bzip2::Compression::default(),
            Compression::Maximum => bzip2::Compression::best(),
            Compression::Minimum => bzip2::Compression::fast(),
        }
    }
}

impl From<Compression> for flate2::Compression {
    fn from(c: Compression) -> flate2::Compression {
        match c {
            Compression::Default => flate2::Compression::default(),
            Compression::Maximum => flate2::Compression::best(),
            Compression::Minimum => flate2::Compression::fast(),
        }
    }
}
