pub type ScanCode = u32;
pub type Timestamp = u32;

pub mod timeline;
pub use timeline::*;

pub mod input_event;
pub use input_event::*;

pub mod pattern;
pub use pattern::*;

pub mod err;
pub use err::*;

// maybe parametrize it with state as well?
pub struct Controller2<I> {
    tl: Timeline<InputEvent<I>>,
}

impl Controller2<KeyboardInput> {
    pub fn press(&mut self, sc: ScanCode, t: Timestamp) {
        self.tl.push(InputEvent::pressed(sc, t))
    }

    pub fn release(&mut self, sc: ScanCode, t: Timestamp) {
        self.tl.push(InputEvent::released(sc, t))
    }
}
