use std::collections::VecDeque;

use bevy::utils::HashMap;

//#[derive(Bundle)]
#[derive(Default)]
pub struct Controller {
    pressed: HashMap<u32, f64>,         //sc and the time when it was pressed
    released: HashMap<u32, (f64, f64)>, //sc, time when it was released and time when it was pressed before this release
    mode: Mode,
    //cursor: Cursor, //isn't it part of the state?
}

impl Controller {
    pub fn press(&mut self, sc: &u32, time: f64) {
        self.released.remove(&sc);

        self.pressed.entry(*sc).or_insert(time);
    }

    pub fn release(&mut self, sc: &u32, time: f64) {
        if let Some(t) = self.pressed.remove(sc) {
            //in some rare cases a key can be actually 'released' but not 'pressed' before it
            //so this is kinda broken
            self.released.insert(*sc, (time, t));
        }
    }

    pub fn is_pressed(&self, sc: u32) -> bool {
        self.pressed.contains_key(&sc)
    }

    ///returns None if the key wasn't pressed or is released
    pub fn get_pressed_duration(&self, sc: u32, time: f64) -> Option<f64> {
        self.pressed.get(&sc).map(|t| time - t)
    }

    pub fn print_dbg(&self) {
        dbg!(&self.pressed);
        dbg!(&self.released);
    }
    //pub fn get_input(&mut self)->&mut Vec
}

enum Mode {
    Normal,
    Insert,
    Visual,
    Halted,
}

impl Default for Mode {
    fn default() -> Self {
        TajMode::Normal
    }
}
