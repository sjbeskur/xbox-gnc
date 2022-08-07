mod xbox;

fn main() {
    println!("Hello, world!");
    let mut gp = xbox::GamePad::new("/dev/input/js0");
    gp.set_callback(simple_callback);

    println!("gp: {:?}", gp);

    let rst = gp.read_device();
    println!("rst: {:?}", rst);

}




fn simple_callback(){
    println!("Hello, world!");
}