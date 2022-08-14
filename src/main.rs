mod xbox;

fn main() {
    let mut gp = xbox::GamePad::new("/dev/input/js0");
    gp.button_handler(button_event);
    gp.axis_handler(axis_event);

    println!("gp: {:?}", gp);

    let rst = gp.read_device();
    println!("rst: {:?}", rst);

}


fn button_event(id: u8){
    println!("button id: {:?}", id);
}

fn axis_event(id: u8, x: i16, y: i16){
    println!("axis id: {:?} - (x,y) {:?} , {:?}", id, x, y);
}