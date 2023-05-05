use bitflags::bitflags;
use clap::ValueEnum;
use gilrs::Button;
use integral_enum::integral_enum;

use crate::gamepads::Vec2;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Keys: u64 {
        const A           = 1 << 0;
        const B           = 1 << 1;
        const Y           = 1 << 2;
        const X           = 1 << 3;

        const LEFT_STICK  = 1 << 4;
        const RIGHT_STICK = 1 << 5;

        const LEFT        = 1 << 6;
        const RIGHT       = 1 << 7;

        const BACK_LEFT  = 1 << 8;
        const BACK_RIGHT = 1 << 9;

        const PLUS       = 1 << 10;
        const MINUS      = 1 << 11;

        const DPAD_LEFT  = 1 << 12;
        const DPAD_UP    = 1 << 13;
        const DPAD_RIGHT = 1 << 14;
        const DPAD_DOWN  = 1 << 15;
    }
}

#[integral_enum]
#[derive(ValueEnum)]
pub enum ControllerType {
    ProController = 1,
    JoyConL = 2,
    JoyConR = 3,
}

#[derive(Debug)]
pub struct Controller {
    pub type_: ControllerType,
    pub keys: Keys,

    pub joy_left: Vec2<i32>,
    pub joy_right: Vec2<i32>,
}

impl From<Button> for Keys {
    fn from(value: Button) -> Self {
        match value {
            Button::DPadDown => Self::DPAD_DOWN,
            Button::DPadLeft => Self::DPAD_LEFT,
            Button::DPadRight => Self::DPAD_RIGHT,
            Button::DPadUp => Self::DPAD_UP,

            Button::RightThumb => Self::RIGHT_STICK,
            Button::LeftThumb => Self::LEFT_STICK,

            Button::East => Self::A,
            Button::West => Self::X,

            Button::North => Self::Y,
            Button::South => Self::B,

            Button::LeftTrigger => Self::LEFT,
            Button::RightTrigger => Self::RIGHT,

            Button::LeftTrigger2 => Self::BACK_LEFT,
            Button::RightTrigger2 => Self::BACK_RIGHT,

            Button::Start => Self::PLUS,
            Button::Select => Self::MINUS,

            _ => Self::empty(),
        }
    }
}
