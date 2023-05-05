use gilrs::GamepadId;
use std::collections::HashMap;

const MAX: usize = 3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}

#[derive(Default)]
pub struct GamepadAxis {
    pub left: Vec2,
    pub right: Vec2,
}

pub struct GamepadState {
    pub axis: GamepadAxis,
    pub index: usize,
}

#[derive(Default)]
pub struct Gamepads {
    map: HashMap<GamepadId, GamepadState>,
    last: usize,
    q: Vec<GamepadState>,
}

impl Vec2<f32> {
    pub fn into_i32_multiplied(self, x_mul: f32, y_mul: f32) -> Vec2<i32> {
        Vec2 {
            x: (self.x * x_mul) as i32,
            y: (self.y * y_mul) as i32,
        }
    }
}

impl Gamepads {
    pub fn index_of(&mut self, id: GamepadId) -> &mut GamepadState {
        self.map.get_mut(&id).unwrap()
    }

    pub fn remove(&mut self, id: GamepadId) {
        if let Some(prev) = self.map.remove(&id) {
            self.q.push(GamepadState {
                axis: Default::default(),
                index: prev.index,
            });
        }
    }

    pub fn insert(&mut self, id: GamepadId) -> &GamepadState {
        let prev = if let Some(i) = self.q.pop() {
            i
        } else if self.last == MAX {
            panic!("Maximum gamepads reached")
        } else {
            let prev = self.last;
            self.last += 1;
            GamepadState {
                axis: Default::default(),
                index: prev,
            }
        };

        &*self.map.entry(id).or_insert(prev)
    }

    pub fn new() -> Self {
        Self::default()
    }
}
