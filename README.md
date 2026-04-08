# Xbox 360 Gamepad (xbox-gnc)

A Rust library for reading Xbox 360 controller input on Linux via the joystick device interface.

## Prerequisites

- Linux (Ubuntu)
- Rust / Cargo

The `xpad` kernel module is built into Linux and loads automatically when an Xbox 360 controller is plugged in — no driver install required.

## Build

```bash
cargo build
```

## Run

1. Plug in your controller, then verify the device is present:
   ```bash
   ls /dev/input/js*
   ```

2. Run the basic example:
   ```bash
   cargo run --example basic
   ```

3. Press **Enter** to stop.

## Troubleshooting

If `/dev/input/js0` doesn't appear, your controller may not be supported by the `xpad` kernel module (common with third-party clones). In that case, try the userspace driver:

```bash
sudo apt install xboxdrv
sudo xboxdrv
```

## Resources

- https://support.xbox.com/en-US/help/xbox-360/accessories/controllers
