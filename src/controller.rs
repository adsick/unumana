mod mode;
pub use mode::*;

use bevy::{prelude::Component, utils::HashMap};

//#[derive(Bundle)]
#[derive(Default, Component)]
pub struct Controller {
    pressed: HashMap<u32, f64>,  //sc and the time when it was pressed
    released: HashMap<u32, f64>, //sc, time when it was released and time when it was pressed before this release
    last_pressed: (u32, f64),
    last_released: (u32, f64),
    pub mode: Mode,
}

impl Controller {
    pub fn press(&mut self, sc: &u32, time: f64) -> ((u32, f64), (u32, f64)) {
        
        self.released.remove(&sc);
        let last_released = self.last_released;
        let last_pressed = self.last_pressed;
        if !self.pressed.contains_key(sc){
            self.pressed.insert(*sc, time);
            self.last_pressed = (*sc, time);
        }
        (last_released, last_pressed)
    }

    pub fn release(&mut self, sc: &u32, time: f64) -> Option<((u32, f64), (u32, f64), f64)> {
        if let Some(t) = self.pressed.remove(sc) {
            let last_pressed = self.last_pressed;
            let last_released = self.last_released;
            self.released.insert(*sc, time);
            self.last_released = (*sc, time);
            return Some((last_pressed, last_released, t));
        }
        return None;
    }

    pub fn is_pressed(&self, sc: u32) -> bool {
        self.pressed.contains_key(&sc)
    }

    ///returns None if the key wasn't pressed or is released
    pub fn get_pressed_duration(&self, sc: u32, time: f64) -> Option<f64> {
        self.pressed.get(&sc).map(|t| time - t)
    }

    pub fn together(&self, sc1: u32, sc2: u32) -> bool {
        if let Some(t1) = self.pressed.get(&sc1) {
            if let Some(t2) = self.pressed.get(&sc2) {
                return t1 < t2;
            }
        }
        return false;
    }

    pub fn print_dbg(&self) {
        use crate::keymap::Convert;
        print!("pressed: ");
        let pressed = self
            .pressed
            .iter()
            .map(|(sc, _)| format!("{}({:?})", sc, sc.dvorak()))
            .collect::<Vec<String>>()
            .join("; ");
        println!("{}", pressed);

        // print!("released: ");
        // let released = self
        //     .released
        //     .iter()
        //     .map(|(sc, _)| format!("{}({:?})", sc, sc.dvorak()))
        //     .collect::<Vec<String>>()
        //     .join("; ");
        // println!("{}", released);
    }
}

#[deprecated]
trait Replacable
where
    Self: Sized + PartialEq,
{
    fn replace_if_eq(self, val: Self, default: Self) -> Self {
        if self == val {
            return default;
        }
        return self;
    }
}
//?? ?????? ???????? ?? ???????? "????????????????????????????" ???????????????????? ?????? ???????? ??????????, ?????????????? ???????????????? ?????? ???????????????????????
impl<T: PartialEq> Replacable for T {}
