# Xbox 360 Gamepad (xbox-gnc)

A Rust library for reading Xbox 360 controller input on Linux via the joystick device interface.

## Prerequisites

- Linux (Ubuntu)
- Rust / Cargo
- `xboxdrv` — userspace Xbox 360 driver

### Install xboxdrv

```bash
sudo apt install xboxdrv
```

## Build

```bash
cargo build
```

## Run

1. Start the driver (connect your controller first):
   ```bash
   sudo xboxdrv
   ```

2. Run the basic example:
   ```bash
   cargo run --example basic
   ```

3. Press **Enter** to stop.

## Resources

- https://xboxdrv.gitlab.io/
- https://support.xbox.com/en-US/help/xbox-360/accessories/controllers
