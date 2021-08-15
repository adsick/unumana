use crate::Command;

pub struct Backend {
    line: usize,                //current line
    text: Vec<(String, usize)>, //last valid utf-8 byte indx
}

impl Default for Backend {
    fn default() -> Self {
        Backend {
            line: 0,
            text: vec![(String::new(), 0)],
        }
    }
}
impl Backend {
    pub fn execute(&mut self, c: &Command) -> () {
        match c {
            Command::MoveCursorRight => self.move_cursor_right(),
            Command::MoveCursorLeft => self.move_cursor_left(),
            Command::MoveCursorForward => self.move_cursor_forward(),
            Command::MoveCursorBackward => self.move_cursor_backward(),

            Command::PutCharAfterCursor(ch) => {
                self.put_char_after_cursor(*ch);
            }

            Command::NewLineAfter => self.new_line_after(),
            Command::NewLineBefore => self.new_line_before(),

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
        self.print_position()

    }

    //imma not sure if this should be public
    fn put_char_after_cursor(&mut self, ch: char) {
        if let Some((line, cur_ind)) = self.text.get_mut(self.line) {
            line.insert(*cur_ind, ch);
            *cur_ind += ch.len_utf8();
        } else {
            panic!("cursor is out of lines");
        }
    }

    fn new_line_after(&mut self) {
        self.text.insert(self.line + 1, (String::new(), 0));
        self.line += 1;
    }

    fn new_line_before(&mut self) {
        self.text.insert(self.line, (String::new(), 0));
    }

    fn remove_char_under_cursor(&mut self) {
        if let Some((line, cur_ind)) = self.text.get_mut(self.line) {
            if line.len() == 0 {
                return;
            }
            prev(line, cur_ind);
            line.remove(*cur_ind);
        }
    }

    //safety: we assume that cur_ind is valid.
    fn move_cursor_right(&mut self) {
        if let Some((line, cur_ind)) = self.text.get_mut(self.line) {
            if let Some(ch) = line.get(*cur_ind..).expect("you are doomed").chars().next() {
                *cur_ind += ch.len_utf8();
            }
            let cur_ind = *cur_ind;
        }
    }

    fn move_cursor_left(&mut self) {
        if let Some((line, cur_ind)) = self.text.get_mut(self.line) {
            prev(line, cur_ind);
        }
    }
    //refactor to use existing methods. just check if on the end on the string.
    fn move_cursor_forward(&mut self) {
        if let Some((line, cur_ind)) = self.text.get_mut(self.line) {
            if line.len() == *cur_ind{
                if self.text.len() > self.line + 1 {
                    self.line += 1;
                }
            } else {
                self.move_cursor_right()
            }
        }
    }

    fn move_cursor_backward(&mut self) {
        if let Some((line, cur_ind)) = self.text.get_mut(self.line) {
            if* cur_ind == 0{
                if self.line > 0 {
                    self.line -= 1;
                }
            } else {
                self.move_cursor_left()
            }
        }
    }


    fn move_cursor_to(&mut self, pos: (usize, usize)) -> (usize, usize) {
        todo!() //use something like self.lines.iter_mut().take(self.cursor.line).0.iter().take(self.cursor.column)...
    }

    //I want to use iterator here, but it pisses me off omg...
    pub fn lines(&self) -> &[(String, usize)] {
        &self.text
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn print_position(&self){
        println!("({}, {})", self.line, self.text.get(self.line).unwrap().1)
    }

    pub fn render(&self) -> String {
        self.text
            .iter()
            .map(|item| item.0.to_owned())
            .collect::<Vec<String>>()
            .join("\n")
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
