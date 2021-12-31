use crate::{ApplicationDescriptor, GattApplication};

include!("../../../resources/services/ping_pong.inc");

pub fn application_descriptor() -> ApplicationDescriptor {
    ApplicationDescriptor::new(SERVICE_UUID, SERVICE_NAME, vec![CHARACTERISTIC_UUID])
}

pub fn gatt_application() -> GattApplication {
    GattApplication::from(application_descriptor())
}
