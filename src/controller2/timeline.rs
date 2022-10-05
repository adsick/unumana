use super::*;

pub struct Timeline<E> {
    events: Vec<E>,
}

impl<E> Default for Timeline<E> {
    fn default() -> Self {
        Self {
            events: Default::default(),
        }
    }
}

impl<E> Timeline<E> {
    pub fn push(&mut self, ev: E) {
        self.events.push(ev)
    }

    pub fn pop(&mut self) -> Option<E> {
        self.pop()
    }

    pub fn clear(&mut self) {
        self.clear()
    }

    pub fn events(&self) -> &[E] {
        &self.events
    }
}
