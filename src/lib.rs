use bevy::ecs::bundle::Bundle;

mod backend;
mod command;
mod controller;
mod keymap;
pub use backend::*;
pub use command::*;
pub use controller::*;
pub use keymap::*;

#[derive(Bundle, Default)]
pub struct Taj {
    controller: Controller,
    backend: Backend,
}

impl Taj {
    pub fn new() -> Self {
        //it probably will take filenames in the future
        Taj::default()
        //Taj{controller: Controller::default(), backend: Backend::default()}
    }
}

pub struct TimedInput {
    pub time: f64,
    pub scan_code: u32,
    pub pressed: bool,
}

#[derive(Default)]
pub struct Cursor {
    line: usize,
    column: usize,
}
