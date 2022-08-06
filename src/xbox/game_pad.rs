use std::fs::File;
use std::io::{Read};
//use byteorder::{LittleEndian, ReadBytesExt};

#[allow(dead_code)]

#[derive(Debug,  Clone)]
pub struct GamePad {
    dev_input: String,
}

pub struct Axis{
    x: i16,
    y: i16,
}

impl GamePad {
    pub fn new(device_path: &str) -> GamePad {
        Self { 
            dev_input: String::from(device_path)
        }
    }

    pub fn read_device(&self) -> Result<(), Box<dyn std::error::Error>> {
        const BUFFER_SIZE: usize = 8;
        let mut buffer = [0u8; BUFFER_SIZE];        
        let mut file = File::open(self.dev_input.clone())?;

        let mut raxis = Axis{x:0, y:0};
        let mut laxis = Axis{x:0, y:0};
        loop {

            let count = file.read(&mut buffer)?;
            // do something with
            self.process(buffer, &mut laxis, &mut raxis);

            println!("X1 {}: Y1: {}  X2 {}: Y2: {} ", laxis.x, laxis.y, raxis.x, raxis.y );

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
            println!("this is a Button Event");                
        }

        println!("address: {}, count: {}, buffer: {:?}", address, message.len(), message);        


        // IsAxis - 0x01 in byte 6 means it is a Axis
        if GamePad::is_flag_set(message[6], 0x02){
            //println!("this is a Axis Event");                
            //let mut axis_values = &message[3..5];
            let axis_values: [u8;2] = [ message[4], message[5] ];
            
            //let mut axis_value = (message[4] as u16) << 8 | message[5] as u16;
            if address == 0 {  // Y1
                //println!("X1: {:?} size: {}", axis_values.read_u16::<LittleEndian>().unwrap(), axis_values.len());
                l_axis.x = i16::from_le_bytes(axis_values);
            }
            if address == 1 {  // Y1
                //println!("X1: {:?} size: {}", axis_values.read_u16::<LittleEndian>().unwrap(), axis_values.len());
                l_axis.y = -i16::from_le_bytes(axis_values);
            }

            if address == 2 {  // Y1
                //println!("X1: {:?} size: {}", axis_values.read_u16::<LittleEndian>().unwrap(), axis_values.len());
                r_axis.x = i16::from_le_bytes(axis_values);
            }
            if address == 3 {  // Y1
                //println!("X1: {:?} size: {}", axis_values.read_u16::<LittleEndian>().unwrap(), axis_values.len());
                r_axis.y = -i16::from_le_bytes(axis_values);
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