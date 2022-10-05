use crate::controller2::*;

pub fn press(code: ScanCode) -> impl Fn(&Kbd) -> PatResult<&Kbd, &InputEvent<KeyboardInput>> {
    move |input: &Kbd| {
        let event = input.first().ok_or(Err::Incomplete)?;
        if event.i.0 == code {
            Ok((&input[1..], &input[0]))
        } else {
            Err(Err::Error(PatternError {
                input,
                kind: PatternErrorKind::Press(code),
            }))
        }
    }
}
