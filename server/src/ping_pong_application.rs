use blt::{ApplicationDescriptor, GattApplication};

include!("../../resources/services/ping_pong.inc");

pub fn gatt_application() -> GattApplication {
    GattApplication::from(ApplicationDescriptor::new(
        SERVICE_UUID,
        SERVICE_NAME,
        vec![CHARACTERISTIC_UUID],
    ))
}
