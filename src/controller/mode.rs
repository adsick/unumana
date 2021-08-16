#[derive(PartialEq, Debug)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Halted,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}
