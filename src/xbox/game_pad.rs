use std::fs::File;
use std::io::{Read};
//use byteorder::{LittleEndian, ReadBytesExt};

#[allow(dead_code)]


type ButtonCallback = fn(id: u8);
type AxisChangedCallback = fn(Axis, Axis);

#[derive(Debug,  Clone)]
pub struct GamePad {
    dev_input: String,

    button_handler: Option<ButtonCallback>,
    axis_handler: Option<AxisChangedCallback>,

}
#[derive(Debug, Clone)]
pub struct Axis{
    pub pad: char,
    pub x: i16,
    pub y: i16,
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


    pub fn read_device(&self) -> Result<(), Box<dyn std::error::Error>> {
        const BUFFER_SIZE: usize = 8;
        let mut buffer = [0u8; BUFFER_SIZE];        
        let mut file = File::open(self.dev_input.clone())?;

        let mut raxis = Axis{pad: 'r', x:0, y:0};
        let mut laxis = Axis{pad: 'l', x:0, y:0};

        loop {

            let count = file.read(&mut buffer)?;
            // do something with
            self.process(buffer, &mut laxis, &mut raxis);
            if count != 8 { break; }            
        }
        Ok(())
    }

    pub fn process(&self, message: [u8;8], l_axis: &mut Axis, r_axis: &mut Axis){

        // ButtonAddress -  address is in byte 7
        let address = message[7]; 

        // HasConfiguration - 0x80 in byte 6 means it has Configuration information
        if GamePad::is_flag_set(message[6], 0x80){
            println!("has a configuration");                
        }

        // IsButton - 0x01 in byte 6 means it is a Button
        if GamePad::is_flag_set(message[6], 0x01) {
            if self.button_handler.is_some() {
                self.button_handler.unwrap()(address);
            }
        }

        println!("address: {}, count: {}, buffer: {:?}", address, message.len(), message);        


        // IsAxis - 0x01 in byte 6 means it is a Axis
        if GamePad::is_flag_set(message[6], 0x02){
            //println!("this is a Axis Event");                
            //let mut axis_values = &message[3..5];
            let axis_values: [u8;2] = [ message[4], message[5] ];
            
            // Left AxisChanged
            if address == 0 { l_axis.x = i16::from_le_bytes(axis_values); }
            if address == 1 { l_axis.y = -i16::from_le_bytes(axis_values); }            
            // println!("this is a Axis Event");        

            // Right AxisChanged
            if address == 2 { r_axis.x = i16::from_le_bytes(axis_values);  }
            if address == 3 { r_axis.y = -i16::from_le_bytes(axis_values); }

            if self.axis_handler.is_some() {
                self.axis_handler.unwrap()(l_axis.clone(),r_axis.clone());
            }

        }
        
        // IsButtonPressed - byte 4 contains the status (0x01 means pressed, 0x00 means released)
        if message[4] == 0x01 {
            println!("pressed or released");                
        }
                
        // AxisValue (value)

    }


    fn is_flag_set(value: u8, flag: u8) -> bool {
        return value & flag == flag;
    }


}