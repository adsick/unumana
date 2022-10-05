use super::ScanCode;
pub type PatResult<I, O, E = PatternError<I>> = Result<(I, O), Err<E>>;

#[derive(Debug)]
pub struct PatternError<I>{
    pub input: I,
    pub kind: PatternErrorKind,
}


#[derive(Debug)]
pub enum PatternErrorKind {
    Press(ScanCode),
}

#[derive(Debug)]
pub enum Err<E>{
    Incomplete,
    Error(E),
}