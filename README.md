# Phonendo reader

## What is this?

A band reader for [phonendo](https://github.com/dltcafe/phonendo) development.

It includes an emulator that allows to generate a continuous stream of random events (specifically, heartbeats) and
transmit them via bluetooth.

The implementation of the heart rate server has been done according to the official specification of this profile
(docs/HRP_V10.pdf).

This project is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
But don't worry about the above if you don't have experience working with workspaces.
To build the workspace (or in other words, each of the crates in it), just run `cargo build` in the root directory.

## Packages in the workspace

### blt

Library crate to manage Bluetooth operations.

### reader

Binary crate that server a Bluetooth GATT server (`APP_MODE=server`) or connects to a Bluetooth GATT server
(`APP_MODE=client`).

To run it: `APP=AplicationName APP_MODE=<server,client> cargo run -p reader`.

Note that there are several applications available, namely ['ping_pong', 'adder', 'cts', 'heart_rate']. However, most of
these applications have been created in order to test bluetooth and libraries and are kept in this repository in order
to have examples that may be useful for the addition of new features in the future.

## Supported devices

### PineTime (InfiniTime)

**BLE Documentation**

https://github.com/InfiniTimeOrg/InfiniTime/blob/develop/doc/ble.md

**GATT info**

```
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0006
	00001801-0000-1000-8000-00805f9b34fb
	Generic Attribute Profile
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0006/char0007
	00002a05-0000-1000-8000-00805f9b34fb
	Service Changed
[NEW] Descriptor (Handle 0x3520)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0006/char0007/desc0009
	00002902-0000-1000-8000-00805f9b34fb
	Client Characteristic Configuration
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service000a
	0000180a-0000-1000-8000-00805f9b34fb
	Device Information
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service000a/char000b
	00002a29-0000-1000-8000-00805f9b34fb
	Manufacturer Name String
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service000a/char000d
	00002a24-0000-1000-8000-00805f9b34fb
	Model Number String
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service000a/char000f
	00002a25-0000-1000-8000-00805f9b34fb
	Serial Number String
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service000a/char0011
	00002a26-0000-1000-8000-00805f9b34fb
	Firmware Revision String
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service000a/char0013
	00002a27-0000-1000-8000-00805f9b34fb
	Hardware Revision String
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service000a/char0015
	00002a28-0000-1000-8000-00805f9b34fb
	Software Revision String
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0017
	00001805-0000-1000-8000-00805f9b34fb
	Current Time Service
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0017/char0018
	00002a2b-0000-1000-8000-00805f9b34fb
	Current Time
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a
	00000000-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char001b
	00000001-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Descriptor (Handle 0xc740)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char001b/desc001d
	00002902-0000-1000-8000-00805f9b34fb
	Client Characteristic Configuration
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char001e
	00000002-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0020
	00000004-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0022
	00000003-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0024
	00000005-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0026
	00000006-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0028
	00000007-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char002a
	00000007-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char002c
	00000008-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char002e
	00000009-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0030
	0000000a-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0032
	0000000b-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service001a/char0034
	0000000c-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0036
	00010000-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0036/char0037
	00010001-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0036/char0039
	00010002-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0036/char003b
	00010003-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0036/char003d
	00010004-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service003f
	00001811-0000-1000-8000-00805f9b34fb
	Alert Notification Service
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service003f/char0040
	00002a46-0000-1000-8000-00805f9b34fb
	New Alert
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service003f/char0042
	00020001-78fc-48fe-8e23-433b3a1942d0
	Vendor specific
[NEW] Descriptor (Handle 0xd540)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service003f/char0042/desc0044
	00002902-0000-1000-8000-00805f9b34fb
	Client Characteristic Configuration
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0045
	00001530-1212-efde-1523-785feabcd123
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0045/char0046
	00001532-1212-efde-1523-785feabcd123
	Vendor specific
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0045/char0048
	00001531-1212-efde-1523-785feabcd123
	Vendor specific
[NEW] Descriptor (Handle 0xdc80)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0045/char0048/desc004a
	00002902-0000-1000-8000-00805f9b34fb
	Client Characteristic Configuration
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0045/char004b
	00001534-1212-efde-1523-785feabcd123
	Vendor specific
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service004d
	0000180f-0000-1000-8000-00805f9b34fb
	Battery Service
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service004d/char004e
	00002a19-0000-1000-8000-00805f9b34fb
	Battery Level
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0050
	00001802-0000-1000-8000-00805f9b34fb
	Immediate Alert
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0050/char0051
	00002a06-0000-1000-8000-00805f9b34fb
	Alert Level
[NEW] Primary Service (Handle 0xb090)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0053
	0000180d-0000-1000-8000-00805f9b34fb
	Heart Rate
[NEW] Characteristic (Handle 0xb5eb)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0053/char0054
	00002a37-0000-1000-8000-00805f9b34fb
	Heart Rate Measurement
[NEW] Descriptor (Handle 0xe0c0)
	/org/bluez/hci0/dev_CA_6F_4F_74_19_1B/service0053/char0054/desc0056
	00002902-0000-1000-8000-00805f9b34fb
	Client Characteristic Configuration
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00000000-78fc-48fe-8e23-433b3a1942d0
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00001530-1212-efde-1523-785feabcd123
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00001800-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00001801-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00001802-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00001805-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 0000180a-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 0000180d-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 0000180f-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00001811-0000-1000-8000-00805f9b34fb
[CHG] Device CA:6F:4F:74:19:1B UUIDs: 00010000-78fc-48fe-8e23-433b3a1942d0
[CHG] Device CA:6F:4F:74:19:1B ServicesResolved: yes
```
