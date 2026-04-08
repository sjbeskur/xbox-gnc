default:
    @just --list

check:
    cargo check

build:
    cargo build --examples

test:
    cargo test

run:
    cargo run --example basic

udp-client remote="127.0.0.1:9000":
    cargo run --example udp_client -- {{remote}}

udp-server port="9000":
    cargo run --example udp_server -- {{port}}

clean:
    cargo clean
