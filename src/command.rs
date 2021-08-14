#[derive(Clone)]
pub enum Command {
    MoveCursorRight, //parametrize with count?
    MoveCursorLeft,
    //MoveCursorForward,
    //MoveCursorBackward,
    PutCharAfterCursor(char), //insert a single char,
    //PutStringAfterCursor(&'a str),
    RemoveUnderCursor,

    Sequence(Vec<Command>),
    Repeat { times: u8, command: Box<Command> },
}
