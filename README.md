# hidapi [![Build Status](https://travis-ci.org/Osspial/hidapi-rs.svg?branch=master)](https://travis-ci.org/Osspial/hidapi-rs) [![Version](https://img.shields.io/crates/v/hidapi.svg)](https://crates.io/crates/hidapi) [![License: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](https://github.com/Osspial/hidapi-rs/blob/master/LICENSE.txt) [![Documentation](https://docs.rs/hidapi/badge.svg)](https://docs.rs/hidapi)

This crate provides a rust abstraction over the features of the C library
[hidapi](https://github.com/signal11/hidapi) by signal11. Based off of
[hidapi_rust](https://github.com/ruabmbua/hidapi_rust) by ruabmbua.

# Usage

This crate is on [crates.io](https://crates.io/crates/hidapi) and can be
used by adding `hidapi` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
hidapi = "0.3"
```
# Example

```rust
extern crate hidapi;

let api = hidapi::HidApi::new().unwrap();
// Print out information about all connected devices
for device in &api.devices() {
    println!("{:#?}", device);
}

// Connect to device using its VID and PID
let (VID, PID) = (0x0123, 0x3456);
let device = api.open(VID, PID).unwrap();

// Read data from device
let mut buf = [0u8; 8];
let res = device.read(&mut buf[..]).unwrap();
println!("Read: {:?}", &buf[..res]);

// Write data to device
let buf = [0u8, 1, 2, 3, 4];
let res = device.write(&buf).unwrap();
println!("Wrote: {:?} byte(s)", res);
```

# Documentation
Available at [docs.rs](https://docs.rs/hidapi).