# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build

# Run (requires xboxdrv running and controller connected)
sudo xboxdrv   # start driver first
./target/debug/xbox-gnc

# Test
cargo test
cargo test --lib   # lib tests only

# Check without building
cargo check
```

## Prerequisites

- Linux (Ubuntu)
- `xboxdrv` v0.8.0 — the userspace Xbox 360 controller driver
- Controller exposed at `/dev/input/js0` (default)

## Architecture

This is a Rust library/binary crate that reads raw Xbox 360 gamepad input from a Linux joystick device file.

**Entry point:** [src/main.rs](src/main.rs) — creates a `GamePad`, registers callbacks, then calls `read_device()` which blocks in a loop reading 8-byte event packets.

**Core module:** `src/xbox/` (re-exported via [src/xbox/mod.rs](src/xbox/mod.rs))

- [src/xbox/game_pad.rs](src/xbox/game_pad.rs) — contains `GamePad`, `Axis`, and `ControllerInputs`
  - `GamePad` holds the device path and two optional function-pointer callbacks: `ButtonCallback = fn(id: u8)` and `AxisChangedCallback = fn(Axis, Axis)`
  - `read_device()` opens the joystick device file and reads 8-byte packets in a loop, calling `process()` on each
  - `process()` decodes the raw packet: byte 6 flags distinguish config (0x80), button (0x01), and axis (0x02) events; byte 7 is the address/id; bytes 4-5 are the axis value (little-endian i16); Y-axis values are negated

**Event protocol (8-byte packets from `/dev/input/jsX`):**
- `message[6]` — event type flags: `0x80` = config, `0x01` = button, `0x02` = axis
- `message[7]` — button id or axis address (0=LX, 1=LY, 2=RX, 3=RY)
- `message[4..=5]` — axis value as little-endian i16
- `message[4]` — button state when event is button type (0x01=pressed, 0x00=released)

**Public API (from `lib.rs`):** `xbox::GamePad` and `xbox::Axis` are the two exported types.
