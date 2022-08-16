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

fn axis_event(laxis: xbox::Axis, raxis: xbox::Axis ){
    println!("LeftStick: {{ X1 {}: Y1: {} }}  RightStick: {{ X2 {}: Y2: {} }} ",  laxis.x, laxis.y,  raxis.x, raxis.y );
}
