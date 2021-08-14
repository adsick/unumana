#[derive(Clone)]
pub enum Command {
    PutCharAfterCursor(char), //insert a single char,
    //PutStringAfterCursor(&'a str),
    RemoveUnderCursor,

    Sequence(Vec<Command>),
    Repeat { times: u8, command: Box<Command> },
}
