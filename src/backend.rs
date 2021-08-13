use crate::{Command, Cursor};
use std::collections::VecDeque;

pub struct Backend {
    cursor: Cursor,
    lines: Vec<String>,
    commands: VecDeque<Command>,
}

impl Default for Backend {
    fn default() -> Self {
        Backend {
            cursor: Cursor::default(),
            lines: vec![String::new()],
            commands: Default::default(),
        }
    }
}
impl Backend {
    //it is not async design, fix it in the future
    pub fn push_command(&mut self, c: Command) {
        self.commands.push_back(c);
    }

    pub fn work(&mut self) {
        while let Some(c) = self.commands.pop_front() {
            self.execute(&c)
        }
    }

    pub fn execute(&mut self, c: &Command) -> () {
        match c {
            Command::PutCharAfterCursor(ch) => {
                self.lines
                    .get_mut(self.cursor.line)
                    .expect("cursor is out of lines")
                    .insert(self.cursor.column, *ch);

                self.cursor.column += ch.len_utf8();
            }
            Command::ClearUnderCursor => {
                let col = self.cursor.column;
                let ln = self.cursor.line;
                if let Some(line) = self.lines.get_mut(ln) {
                    if col <= line.len() {
                        if col > 0 {
                            let len = line.remove(col - 1).len_utf8(); //-1 causes panic with russian chars
                            self.cursor.column -= len;
                        }
                    }
                }
            }
            Command::Sequence(v) => {
                for c in v {
                    self.execute(c)
                }
            }
            Command::Repeat { times, command } => {
                for i in 0..*times {
                    self.execute(&command)
                }
            }
        }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}
