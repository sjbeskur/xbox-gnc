/// Reads gamepad input and sends state snapshots via UDP at 20hz.
///
/// Usage: cargo run --example udp_client -- <remote_host:port>
/// e.g.   cargo run --example udp_client -- 192.168.1.10:9000
use std::net::UdpSocket;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::Duration;

use xbox_gnc::xbox::{Axis, ControllerInputs, GamePad, GamePadState};

fn main() {
    let remote = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:9000".to_string());

    let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind socket");
    socket.connect(&remote).expect("failed to connect to remote");
    println!("Sending gamepad state to {}", remote);

    let state = Arc::new(Mutex::new(GamePadState::default()));

    let mut gp = GamePad::new("/dev/input/js0");

    let state_axis = state.clone();
    gp.axis_handler(move |axis: Axis| {
        let mut s = state_axis.lock().unwrap();
        match axis.pad {
            ControllerInputs::LeftStick  => { s.left_stick_x  = axis.x; s.left_stick_y  = axis.y; }
            ControllerInputs::RightStick => { s.right_stick_x = axis.x; s.right_stick_y = axis.y; }
            ControllerInputs::LTrigger   => { s.left_trigger  = axis.x; }
            ControllerInputs::RTrigger   => { s.right_trigger = axis.x; }
            ControllerInputs::DPad       => { s.dpad_x        = axis.x; s.dpad_y        = axis.y; }
            _ => {}
        }
        println!("axis  {:?}: x={} y={}", axis.pad, axis.x, axis.y);
    });

    let state_btn = state.clone();
    gp.button_handler(move |input: ControllerInputs, pressed: bool| {
        let mut s = state_btn.lock().unwrap();
        let bit = GamePadState::button_bit(&input);
        if pressed { s.buttons |= bit; } else { s.buttons &= !bit; }
        println!("button {:?} {}", input, if pressed { "pressed" } else { "released" });
    });

    let stop = Arc::new(AtomicBool::new(false));
    let stop_read = stop.clone();
    let stop_send = stop.clone();

    // Read gamepad on a background thread
    std::thread::spawn(move || {
        if let Err(e) = gp.read_device(stop_read) {
            eprintln!("read_device error: {}", e);
        }
    });

    // Send state snapshots at 20hz, log once per second
    let state_send = state.clone();
    std::thread::spawn(move || {
        let mut tick: u32 = 0;
        while !stop_send.load(Ordering::Relaxed) {
            let snapshot = state_send.lock().unwrap().clone();
            match postcard::to_allocvec(&snapshot) {
                Ok(bytes) => {
                    socket.send(&bytes).ok();
                    if tick % 20 == 0 {
                        println!("sending {} bytes | {:?}", bytes.len(), snapshot);
                    }
                }
                Err(e) => eprintln!("serialize error: {}", e),
            }
            tick = tick.wrapping_add(1);
            std::thread::sleep(Duration::from_millis(50));
        }
    });

    println!("Press Enter to stop...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    stop.store(true, Ordering::Relaxed);
}
