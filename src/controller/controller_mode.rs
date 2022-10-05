#[derive(PartialEq, Debug)]
pub enum ControllerMode {
    Normal,
    Insert,
    Visual,
    Halted,
}

impl Default for ControllerMode {
    fn default() -> Self {
        ControllerMode::Normal
    }
}
