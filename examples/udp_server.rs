/// Receives gamepad state snapshots over UDP and prints them.
/// Run this on the remote device.
///
/// Usage: cargo run --example udp_server -- <bind_port>
/// e.g.   cargo run --example udp_server -- 9000
use std::net::UdpSocket;

use xbox_gnc::xbox::GamePadState;

fn main() {
    let port = std::env::args().nth(1).unwrap_or_else(|| "9000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let socket = UdpSocket::bind(&addr).expect("failed to bind socket");
    println!("Listening on {}", addr);

    let mut buf = [0u8; 64];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                match postcard::from_bytes::<GamePadState>(&buf[..len]) {
                    Ok(state) => println!("[{}] {:?}", src, state),
                    Err(e)    => eprintln!("deserialize error from {}: {}", src, e),
                }
            }
            Err(e) => eprintln!("recv error: {}", e),
        }
    }
}
