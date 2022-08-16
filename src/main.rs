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
    println!("{}: {{ X1 {}: Y1: {} }}  {}: {{ X2 {}: Y2: {} }} ", laxis.pad, laxis.x, laxis.y, raxis.pad, raxis.x, raxis.y );
}
