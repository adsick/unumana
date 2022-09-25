pub use bevy::{input::keyboard::KeyboardInput, prelude::*};
pub use bevy_prototype_lyon::prelude::*;

mod backend;
mod frontend;
mod generic_keymap;
mod fixed_keymap; //may be renamed to just keymap in the future

pub use backend::*;
pub use frontend::*;
pub use generic_keymap::*;
pub use fixed_keymap::*;
//systems
mod setup;
pub use setup::setup;

mod input;
pub use input::input_system;


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

#[derive(Component)]
pub struct Scrollable;

#[derive(Component)]
pub struct Content;

#[derive(Component)]
pub struct StatusLine;