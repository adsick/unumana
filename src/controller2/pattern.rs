use super::{InputEvent, KeyboardInput, PatResult};

pub mod press;
pub use press::*;

pub type Kbd = [InputEvent<KeyboardInput>];

// trying to do something nom-inspired
// consider introducing Pattern and PatternMut
pub trait Pattern<I, O>: PatternMut<I, O> {
    //"parse"
    fn run(&self, input: I) -> PatResult<I, O>;

    fn and<P2, O2>(self, pat2: P2) -> And<Self, P2>
    where
        P2: Pattern<I, O2>, //kinda funny, don't get why P2 does not require Sized bound
        Self: Sized,
    {
        And(self, pat2)
    }

    fn or<P2>(self, pat2: P2) -> Or<Self, P2>
    where
        P2: Pattern<I, O>,
        Self: Sized,
    {
        Or(self, pat2)
    }
}

pub trait PatternMut<I, O>: Sized {
    fn run(&mut self, input: I) -> PatResult<I, O>;
    fn and<P2, O2>(self, pat2: P2) -> And<Self, P2>
    where
        P2: PatternMut<I, O2>, //kinda funny, don't get why P2 does not require Sized bound
        Self: Sized,
    {
        And(self, pat2)
    }

    fn or<P2>(self, pat2: P2) -> Or<Self, P2>
    where
        P2: PatternMut<I, O>,
        Self: Sized,
    {
        Or(self, pat2)
    }
}

pub struct And<P1, P2>(P1, P2);

impl<I, O1, O2, P1, P2> PatternMut<I, (O1, O2)> for And<P1, P2>
where
    P1: Pattern<I, O1>,
    P2: Pattern<I, O2>,
{
    fn run(&mut self, input: I) -> PatResult<I, (O1, O2)> {
        let (input, output1) = self.0.run(input)?;
        let (input, output2) = self.1.run(input)?;
        Ok((input, (output1, output2)))
    }
}

pub struct Or<P1, P2>(P1, P2);

impl<I: Clone, O, P1, P2> PatternMut<I, O> for Or<P1, P2>
where
    P1: Pattern<I, O>,
    P2: Pattern<I, O>,
{
    fn run(&mut self, input: I) -> PatResult<I, O> {
        match self.0.run(input.clone()) {
            Err(_) => {
                match self.1.run(input) {
                    Err(e2) => Err(e2), // not the best error handling
                    r => r,
                }
            }
            r => r,
        }
    }
}

impl<'a, I, O, F> Pattern<I, O> for F
where
    F: Fn(I) -> PatResult<I, O> + 'a,
{
    fn run(&self, input: I) -> PatResult<I, O> {
        self(input)
    }
}

impl<'a, I, O, F> PatternMut<I, O> for F
where
    F: FnMut(I) -> PatResult<I, O> + 'a,
{
    fn run(&mut self, input: I) -> PatResult<I, O> {
        self(input)
    }
}

#[cfg(test)]
mod tests{
    use crate::controller2::*;


    #[test]
    fn basic(){
        let mut tl = Timeline::default();
        tl.push(InputEvent::pressed(1, 1000));
        tl.push(InputEvent::pressed(2, 2000));
        tl.push(InputEvent::pressed(3, 3000));

        let pat1 = press(1);
        let pat2 = press(2);

        let (input, output) = Pattern::and(pat1, pat2).run(tl.events()).unwrap();
        dbg!(input);
        dbg!(output);

        let err = Pattern::or(press(1), press(2)).run(input).unwrap_err();
        dbg!(err);
    }
}