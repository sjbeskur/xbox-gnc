use std::fs::File;
use std::io::Read;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use super::error::GamePadError;
use super::types::{Axis, ControllerInputs};

type ButtonCallback = fn(ControllerInputs);
type AxisChangedCallback = fn(Axis);

#[derive(Debug, Clone)]
pub struct GamePad {
    dev_input: String,
    button_handler: Option<ButtonCallback>,
    axis_handler: Option<AxisChangedCallback>,
}

impl GamePad {
    pub fn new(device_path: &str) -> GamePad {
        Self {
            dev_input: String::from(device_path),
            button_handler: None,
            axis_handler: None,
        }
    }

    pub fn button_handler(&mut self, callback: ButtonCallback) {
        self.button_handler = Some(callback);
    }

    pub fn axis_handler(&mut self, callback: AxisChangedCallback) {
        self.axis_handler = Some(callback);
    }

    pub fn read_device(&self, stop: Arc<AtomicBool>) -> Result<(), GamePadError> {
        const BUFFER_SIZE: usize = 8;
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut file = File::open(self.dev_input.clone()).map_err(GamePadError::DeviceOpen)?;

        let mut raxis = Axis {
            pad: ControllerInputs::RightStick,
            x: 0,
            y: 0,
        };
        let mut laxis = Axis {
            pad: ControllerInputs::LeftStick,
            x: 0,
            y: 0,
        };

        loop {
            if stop.load(Ordering::Relaxed) {
                break;
            }
            let count = file.read(&mut buffer).map_err(GamePadError::Read)?;
            self.process(buffer, &mut laxis, &mut raxis);
            if count != 8 {
                break;
            }
        }
        Ok(())
    }

    fn process(&self, message: [u8; 8], l_axis: &mut Axis, r_axis: &mut Axis) {
        // ButtonAddress - address is in byte 7
        let address = message[7];

        // HasConfiguration - 0x80 in byte 6 means it has configuration information
        if GamePad::is_flag_set(message[6], 0x80) {
            println!("has a configuration");
        }

        // IsButton - 0x01 in byte 6 means it is a button
        if GamePad::is_flag_set(message[6], 0x01) {
            if let (Some(cb), Some(input)) = (self.button_handler, GamePad::button_from_id(address))
            {
                cb(input);
            }
            // IsButtonPressed - byte 4: 0x01 = pressed, 0x00 = released
            if message[4] == 0x01 {
                println!("pressed or released");
            }
        }

        // IsAxis - 0x02 in byte 6 means it is an axis event
        if GamePad::is_flag_set(message[6], 0x02) {
            let value = i16::from_le_bytes([message[4], message[5]]);

            let changed = match address {
                0 => {
                    l_axis.pad = ControllerInputs::LeftStick;
                    l_axis.x = value;
                    Some(l_axis.clone())
                }
                1 => {
                    l_axis.pad = ControllerInputs::LeftStick;
                    l_axis.y = -value;
                    Some(l_axis.clone())
                }
                2 => {
                    r_axis.pad = ControllerInputs::RightStick;
                    r_axis.x = value;
                    Some(r_axis.clone())
                }
                3 => {
                    r_axis.pad = ControllerInputs::RightStick;
                    r_axis.y = -value;
                    Some(r_axis.clone())
                }
                6 => Some(Axis {
                    pad: ControllerInputs::DPad,
                    x: value,
                    y: 0,
                }),
                7 => Some(Axis {
                    pad: ControllerInputs::DPad,
                    x: 0,
                    y: -value,
                }),
                _ => None,
            };

            if let (Some(cb), Some(axis)) = (self.axis_handler, changed) {
                cb(axis);
            }
        }
    }

    fn button_from_id(id: u8) -> Option<ControllerInputs> {
        match id {
            0 => Some(ControllerInputs::AButton),
            1 => Some(ControllerInputs::BButton),
            2 => Some(ControllerInputs::XButton),
            3 => Some(ControllerInputs::YButton),
            4 => Some(ControllerInputs::LBumper),
            5 => Some(ControllerInputs::RBumper),
            6 => Some(ControllerInputs::LTrigger),
            7 => Some(ControllerInputs::RTrigger),
            _ => None,
        }
    }

    fn is_flag_set(value: u8, flag: u8) -> bool {
        value & flag == flag
    }
}
