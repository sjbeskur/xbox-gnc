use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControllerInputs {
    LeftStick,
    RightStick,
    XButton,
    YButton,
    AButton,
    BButton,
    LTrigger,
    RTrigger,
    LBumper,
    RBumper,
    DPad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axis {
    pub pad: ControllerInputs,
    pub x: i16,
    pub y: i16,
}

/// Full controller state snapshot, suitable for sending over the network.
/// Buttons are stored as a bitmask — use [`GamePadState::button_bit`] to
/// set/clear individual buttons.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GamePadState {
    pub left_stick_x:  i16,
    pub left_stick_y:  i16,
    pub right_stick_x: i16,
    pub right_stick_y: i16,
    pub left_trigger:  i16,
    pub right_trigger: i16,
    pub dpad_x:        i16,
    pub dpad_y:        i16,
    /// Bitmask of currently pressed buttons.
    pub buttons: u16,
}

impl GamePadState {
    pub fn button_bit(input: &ControllerInputs) -> u16 {
        match input {
            ControllerInputs::AButton  => 1 << 0,
            ControllerInputs::BButton  => 1 << 1,
            ControllerInputs::XButton  => 1 << 2,
            ControllerInputs::YButton  => 1 << 3,
            ControllerInputs::LBumper  => 1 << 4,
            ControllerInputs::RBumper  => 1 << 5,
            ControllerInputs::LTrigger => 1 << 6,
            ControllerInputs::RTrigger => 1 << 7,
            _                          => 0,
        }
    }

    pub fn is_pressed(&self, input: &ControllerInputs) -> bool {
        self.buttons & GamePadState::button_bit(input) != 0
    }
}
