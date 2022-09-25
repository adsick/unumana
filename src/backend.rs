use bevy::prelude::Component;

use crate::Command;

pub struct CurrentLine(usize);
struct TextBuffer {
    lines: Vec<Line>,
}

impl TextBuffer {
    pub fn new() -> Self {
        TextBuffer {
            lines: vec![Line::new()],
        }
    }
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }
    pub fn get_line_mut(&mut self, index: usize) -> Option<&mut Line> {
        self.lines.get_mut(index)
    }

    /// allocate a new line at the specified index
    pub fn new_line(&mut self, index: usize){
        self.lines.insert(index, Line::new())
    }
}
pub struct Line(String, usize);

impl Line {
    pub fn new() -> Self {
        Line(String::new(), 0)
    }
}
#[derive(Component)]
pub struct Backend {
    line: CurrentLine, //current line
    text: TextBuffer,
}

impl Default for Backend {
    fn default() -> Self {
        Backend {
            line: CurrentLine(0),
            text: TextBuffer::new(),
        }
    }
}
impl Backend {
    pub fn execute(&mut self, c: &Command) -> () {
        match c {
            Command::MoveCursorRightward => self.move_cursor_rightward(),
            Command::MoveCursorLeftward => self.move_cursor_leftward(),

            Command::MoveCursorUpward => self.move_cursor_upward(),
            Command::MoveCursorDownward => self.move_cursor_downward(),

            Command::MoveCursorForward => self.move_cursor_forward(),
            Command::MoveCursorBackward => self.move_cursor_backward(),

            Command::MoveCursorToTheFirstChar => self.move_cursor_to_the_first_char(),
            Command::MoveCursorToTheEndOfTheLine => self.move_cursor_to_the_end_of_the_line(),

            Command::PutCharAfterCursor(ch) => {
                self.put_char_after_cursor(*ch);
            }

            Command::NewLineAfter => self.new_line_after(),
            Command::NewLineBefore => self.new_line_before(),

            Command::RemoveCharBeforeCursor => self.remove_char_before_cursor(),
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
        if let Some(Line(line, cur_ind)) = self.text.get_line_mut(self.line.0) {
            line.insert(*cur_ind, ch);
            *cur_ind += ch.len_utf8();
        } else {
            panic!("cursor is out of lines"); //todo move it to the right position
        }
    }

    fn new_line_after(&mut self) {
        self.text.new_line(self.line() + 1);
        self.line.0 += 1;
    }

    fn new_line_before(&mut self) {
        self.text.lines.insert(self.line.0, Line::new());
    }

    fn remove_char_before_cursor(&mut self) {
        if let Some(Line(line, cur_ind)) = self.text.lines.get_mut(self.line.0) {
            if line.len() == 0 {
                return;
            }
            prev(line, cur_ind);
            line.remove(*cur_ind);
        }
    }

    //safety: we assume that cur_ind is valid.
    fn move_cursor_rightward(&mut self) {
        if let Some(Line(line, cur_ind)) = self.text.lines.get_mut(self.line.0) {
            if let Some(ch) = line.get(*cur_ind..).unwrap().chars().next() {
                *cur_ind += ch.len_utf8();
            }
        }
    }

    fn move_cursor_leftward(&mut self) {
        if let Some(Line(line, cur_ind)) = self.text.lines.get_mut(self.line.0) {
            prev(line, cur_ind);
        }
    }

    fn move_cursor_upward(&mut self) {
        if self.line.0 > 0 {
            self.line.0 -= 1;
        }
    }

    fn move_cursor_downward(&mut self) {
        if self.line.0 + 1 < self.text.len() {
            self.line.0 += 1;
        }
    }

    //refactor to use existing methods. just check if on the end on the string.
    fn move_cursor_forward(&mut self) {
        if let Some(Line(line, cur_ind)) = self.text.get_line_mut(self.line.0) {
            if line.len() == *cur_ind {
                if self.text.len() > self.line.0 + 1 {
                    self.line.0 += 1;
                }
            } else {
                self.move_cursor_rightward()
            }
        }
    }

    fn move_cursor_backward(&mut self) {
        if let Some(Line(_, cur_ind)) = self.text.get_line_mut(self.line.0) {
            if *cur_ind == 0 {
                if self.line.0 > 0 {
                    self.line.0 -= 1;
                }
            } else {
                self.move_cursor_leftward()
            }
        }
    }

    fn move_cursor_to_the_end_of_the_line(&mut self) {
        if let Some(Line(s, i)) = self.text.get_line_mut(self.line.0) {
            *i = s.len();
        }
    }

    fn move_cursor_to_the_first_char(&mut self) {
        if let Some(Line(_, i)) = self.text.get_line_mut(self.line.0) {
            *i = 0;
        }
    }

    #[allow(unused)]
    fn move_cursor_to(&mut self, pos: (usize, usize)) -> (usize, usize) {
        todo!() //use something like self.lines.iter_mut().take(self.cursor.line).0.iter().take(self.cursor.column)...
    }

    pub fn line(&self) -> usize {
        self.line.0
    }

    pub fn position(&self) -> String {
        format!(
            "({}, {})",
            self.line.0,
            self.text.get_line(self.line.0).unwrap().1
        )
    }

    pub fn render(&self) -> String {
        self.text
            .lines
            .iter()
            .map(|item| item.0.to_owned())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

//mutates cur_ind to sit on previous char
//it is better to use external crates for this I guess
fn prev(str: &str, cur_ind: &mut usize) {
    // if *cur_ind == 0 {
    //     return;
    // }
    // if let Some((ind, _)) = str.get(..*cur_ind).unwrap().char_indices().rev().next() {
    //     *cur_ind = ind;
    // }
    if let Some(Some((ind, _))) = str.get(..*cur_ind).map(|s|s.char_indices().rev().next()) {
        *cur_ind = ind;
    }
}
