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
pub struct Editor {
    controller: Controller,
    backend: Backend,
}

impl Editor {
    pub fn new() -> Self {
        //it probably will take filenames in the future
        Editor::default()
    }
}

// #[derive(Default)]
// pub struct Cursor {
//     line: usize,
//     column: usize,
// }
