use gilrs::GamepadId;
use std::collections::HashMap;

const MAX: usize = 3;

#[derive(Default)]
pub struct Gamepads {
    map: HashMap<GamepadId, usize>,
    last: usize,
    q: Vec<usize>,
}

impl Gamepads {
    pub fn index_of(&self, id: GamepadId) -> usize {
        self.map[&id]
    }

    pub fn remove(&mut self, id: GamepadId) {
        if let Some(prev) = self.map.remove(&id) {
            self.q.push(prev);
        }
    }

    pub fn insert(&mut self, id: GamepadId) -> usize {
        let prev = if let Some(i) = self.q.pop() {
            i
        } else if self.last == MAX {
            panic!("Maximum gamepads reached")
        } else {
            let prev = self.last;
            self.last += 1;
            prev
        };

        self.map.insert(id, prev);

        prev
    }

    pub fn new() -> Self {
        Self::default()
    }
}
