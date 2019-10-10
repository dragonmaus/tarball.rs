#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    Minimal,
    Normal,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Normal
    }
}
