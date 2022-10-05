use super::{ScanCode, Timestamp};


/// Input Event 
/// single piece of user input e.g. key press, key release or mouse move
/// it is generic over input type I
#[derive(Debug, Clone, Copy)]
pub struct InputEvent<I> {
    pub i: I,
    /// time in milliseconds
    t: Timestamp,
}

impl InputEvent<KeyboardInput> {
    pub fn pressed(sc: ScanCode, t: Timestamp) -> Self {
        InputEvent {
            i: KeyboardInput(sc, 1),
            t,
        }
    }

    pub fn released(sc: ScanCode, t: Timestamp) -> Self {
        InputEvent {
            i: KeyboardInput(sc, 0),
            t,
        }
    }

    #[inline]
    pub fn sc(&self) -> ScanCode {
        self.i.0
    }

    #[inline]
    pub fn t(&self) -> Timestamp {
        self.t
    }

    #[inline]
    pub fn p(&self) -> bool {
        self.i.1 > 0
    }

    #[inline]
    pub fn r(&self) -> bool {
        self.i.1 == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardInput(pub ScanCode, pub u32);
