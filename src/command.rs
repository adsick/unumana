#[derive(Clone)]
pub enum Command {
    MoveCursorRightward, //parametrize with count?
    MoveCursorLeftward,
    MoveCursorForward,
    MoveCursorBackward,
    MoveCursorUpward,
    MoveCursorDownward,



    PutCharAfterCursor(char), //insert a single char,
    NewLineAfter,
    NewLineBefore,
    //PutStringAfterCursor(&'a str),
    RemoveUnderCursor,

    Sequence(Vec<Command>),
    Repeat { times: u8, command: Box<Command> },
}
