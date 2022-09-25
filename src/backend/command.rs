//note, some of the command names might be a little bit misleading
//beware.

#[derive(Clone)]
pub enum Command {
    MoveCursorRightward, //parametrize with count?
    MoveCursorLeftward,

    MoveCursorUpward,
    MoveCursorDownward,

    MoveCursorForward,
    MoveCursorBackward,

    MoveCursorToTheFirstChar,
    MoveCursorToTheEndOfTheLine,

    PutCharAfterCursor(char), //insert a single char
    NewLineAfter,
    NewLineBefore,
    //PutStringAfterCursor(&'a str),
    RemoveCharBeforeCursor,

    Sequence(Vec<Command>),
    Repeat { times: u8, command: Box<Command> },
}
