#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Axis {
    pub pad: ControllerInputs,
    pub x: i16,
    pub y: i16,
}
