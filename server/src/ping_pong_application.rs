use bluer::gatt::local::{
    Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod,
    CharacteristicWrite, CharacteristicWriteMethod, Service,
};
use uuid::Uuid;

use blt::application_manager::{ApplicationFactory, GattApplication};

include!("../../resources/services/ping_pong.inc");

#[derive(Default)]
pub struct PingPongApplication {
    service_uuid: Uuid,
    characteristic_uuid: Uuid,
    service_name: &'static str,
}

impl PingPongApplication {
    pub fn new() -> Self {
        Self {
            service_uuid: SERVICE_UUID,
            characteristic_uuid: CHARACTERISTIC_UUID,
            service_name: SERVICE_NAME,
        }
    }
}

impl ApplicationFactory for PingPongApplication {
    fn create(&self) -> GattApplication {
        let (characteristic_control, characteristic_control_handle) =
            bluer::gatt::local::characteristic_control();
        GattApplication::new(
            Application {
                services: vec![Service {
                    uuid: self.service_uuid,
                    primary: true,
                    characteristics: vec![Characteristic {
                        uuid: self.characteristic_uuid,
                        write: Some(CharacteristicWrite {
                            write_without_response: true,
                            method: CharacteristicWriteMethod::Io,
                            ..Default::default()
                        }),
                        notify: Some(CharacteristicNotify {
                            notify: true,
                            method: CharacteristicNotifyMethod::Io,
                            ..Default::default()
                        }),
                        control_handle: characteristic_control_handle,
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            },
            self.service_uuid,
            characteristic_control,
            self.service_name,
        )
    }
}
