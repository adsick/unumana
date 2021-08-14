use crate::{Command, Cursor};

pub struct Backend {
    cursor: Cursor,
    lines: Vec<(String, usize)>, //last valid utf-8 byte indx
}

impl Default for Backend {
    fn default() -> Self {
        Backend {
            cursor: Cursor::default(),
            lines: vec![(String::new(), 0)],
        }
    }
}
impl Backend {
    pub fn execute(&mut self, c: &Command) -> () {
        match c {
            Command::MoveCursorRight => self.move_cursor_right(),
            Command::MoveCursorLeft => self.move_cursor_left(),
            Command::PutCharAfterCursor(ch) => {
                self.put_char_after_cursor(*ch);
            }
            Command::RemoveUnderCursor => self.remove_char_under_cursor(),
            Command::Sequence(v) => {
                for c in v {
                    self.execute(c)
                }
            }
            Command::Repeat { times, command } => {
                for _ in 0..*times {
                    self.execute(&command)
                }
            }
        }
    }

    //imma not sure if this should be public
    fn put_char_after_cursor(&mut self, ch: char) {
        if let Some((line, ind)) = self.lines.get_mut(self.cursor.line) {
            line.insert(*ind, ch);
            *ind += ch.len_utf8();
            //println!("ind: {}", ind);
        } else {
            panic!("cursor is out of lines");
        }

        //self.cursor.column += ch.len_utf8();
    }

    fn remove_char_under_cursor(&mut self) {
        let ln = self.cursor.line;
        if let Some((line, cur_ind)) = self.lines.get_mut(ln) {
            if line.len() == 0 {
                return;
            }
            prev(line, cur_ind);
            line.remove(*cur_ind);
        }
    }

    fn move_cursor_left(&mut self) {
        let ln = self.cursor.line;
        if let Some((line, cur_ind)) = self.lines.get_mut(ln) {
            println!("cur_ind is {}", cur_ind);
            prev(line, cur_ind);
            println!("new_ind is {}", cur_ind);
        }
    }

    //it is probably too early for using unsafe, but...
    //safety: we assume that cur_ind is valid.
    fn move_cursor_right(&mut self) {
        let ln = self.cursor.line;
        if let Some((line, cur_ind)) = self.lines.get_mut(ln) {
            println!("cur_ind is {}", cur_ind);
            if let Some(ch) = line.get(*cur_ind..).expect("you are doomed").chars().next() {
                *cur_ind += ch.len_utf8();
            }
            println!("new_ind is {}", cur_ind);
        }
    }

    fn move_cursor_to(&mut self, pos: (usize, usize)) -> (usize, usize) {
        todo!() //use something like self.lines.iter_mut().take(self.cursor.line).0.iter().take(self.cursor.column)...
    }

    //I want to use iterator here, but it pisses me off omg...
    pub fn lines(&self) -> &[(String, usize)] {
        &self.lines
    }
}

//mutates cur_ind to sit on previous char
//it is better to use external crates for this I guess
fn prev(str: &str, cur_ind: &mut usize) {
    if *cur_ind == 0 {
        return;
    }
    if let Some((ind, _)) = str.get(..*cur_ind).unwrap().char_indices().rev().next() {
        *cur_ind = ind;
    }
}
