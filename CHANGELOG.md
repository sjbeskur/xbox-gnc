# Changelog

## [0.1.2] - 2026-04-07

### Added
- `GamePadError` enum with `DeviceOpen` and `Read` variants replacing `Box<dyn std::error::Error>`
- Cancellation support for `read_device` via `Arc<AtomicBool>` stop flag
- D-pad handling as axis events (addresses 6 and 7)
- `button_from_id` maps raw button ids to `ControllerInputs` enum variants

### Changed
- Reorganized into a library crate; moved binary entry point to `examples/basic.rs`
- Split `game_pad.rs` into `error.rs`, `types.rs`, and `game_pad.rs`
- `ButtonCallback` now receives `ControllerInputs` instead of raw `u8`
- `AxisChangedCallback` now receives only the changed `Axis` instead of both axes
- Axis address mapping uses `match` with `ControllerInputs` variants; `Axis.pad` updated on each event
- `process()` made private
- `read_device` runs on a background thread in the example

### Fixed
- Button pressed/released check was firing on axis events; moved inside button event block
- `LBumber` / `RBumber` typos corrected to `LBumper` / `RBumper`
- Replaced `is_some()` + `unwrap()` with `if let Some(cb)` pattern for callbacks

## [0.1.0] - 2022-08-15

### Added
- `ControllerInputs` enum for controller input types
- Axis event refactoring with left/right stick support
- Button and axis callbacks (`ButtonCallback`, `AxisChangedCallback`)

### Changed
- Removed unused dependencies (`byteorder`, `hey_listen`)

## [0.0.1] - 2022-08-06

### Added
- Initial import with `GamePad` struct reading raw 8-byte packets from `/dev/input/js0`
- Callback proof of concept for button and axis events
