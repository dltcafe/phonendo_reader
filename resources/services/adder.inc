/// Service name
const SERVICE_NAME: &str = "adder";

/// Service UUID for GATT adder.
const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xFEEDC0DE00003);

/// Characteristic UUID for GATT adder.
const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00003);
