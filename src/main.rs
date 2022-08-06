mod xbox;

fn main() {
    println!("Hello, world!");
    let gp = xbox::GamePad::new("/dev/input/js0");

    println!("gp: {:?}", gp);

    let rst = gp.read_device();
    println!("rst: {:?}", rst);
}
