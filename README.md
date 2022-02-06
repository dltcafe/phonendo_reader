# Phonendo emulator

## What is this?

A band emulator for [phonendo](https://github.com/dltcafe/phonendo) development.

This emulator allows to generate a continuous stream of random events (specifically, heartbeats and steps) and transmit it via bluetooth.

The initial code of this project is based on [bluer examples](https://github.com/bluez/bluer/tree/master/bluer/examples).

This project is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
But don't worry about the above if you don't have experience working with workspaces.
To build the workspace (or in other words, each of the crates in it), just run `cargo build` in the root directory.

## Packages in the workspace

### blt

Library crate to manage Bluetooth operations.

### server

Binary crate that serves a Bluetooth GATT server.

To run it: `APP=AplicationName cargo run -p server`.

Note that this is a **WIP**, so right now the only available apps are ['PingPong'].

### client

Binary crate that connects to our Bluetooth GATT server.

To run it: `APP=ApplicationName cargo run -p client`.

Note that this is a **WIP**, so right now the only available apps are ['PingPong'].