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
impl Into<u32> for Compression {
    fn into(self) -> u32 {
        self.into_custom(1, 9, 6)
    }
}

impl Into<bzip2::Compression> for Compression {
    fn into(self) -> bzip2::Compression {
        match self {
            Compression::Default => bzip2::Compression::default(),
            Compression::Maximum => bzip2::Compression::best(),
            Compression::Minimum => bzip2::Compression::fast(),
        }
    }
}

impl Into<flate2::Compression> for Compression {
    fn into(self) -> flate2::Compression {
        match self {
            Compression::Default => flate2::Compression::default(),
            Compression::Maximum => flate2::Compression::best(),
            Compression::Minimum => flate2::Compression::fast(),
        }
    }
}
