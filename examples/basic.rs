use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use xbox_gnc::xbox::{Axis, ControllerInputs, GamePad};

fn main() {
    let mut gp = GamePad::new("/dev/input/js0");
    gp.button_handler(button_event);
    gp.axis_handler(axis_event);

    println!("gp: {:?}", gp);

    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();

    let handle = std::thread::spawn(move || {
        let rst = gp.read_device(stop_clone);
        println!("rst: {:?}", rst);
    });

    println!("Press Enter to stop...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    stop.store(true, Ordering::Relaxed);

    handle.join().unwrap();
}

fn button_event(input: ControllerInputs, pressed: bool) {
    println!("button: {:?} {}", input, if pressed { "pressed" } else { "released" });
}

fn axis_event(axis: Axis) {
    println!("{:?}: {{ x: {}, y: {} }}", axis.pad, axis.x, axis.y);
}
