use bluer::gatt::local::{
    Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod,
    CharacteristicWrite, CharacteristicWriteMethod, Service,
};

use crate::server_manager::GattApplication;

include!("../../resources/service.inc");

pub fn create_application_definition() -> GattApplication {
    let (characteristic_control, characteristic_control_handle) =
        bluer::gatt::local::characteristic_control();
    GattApplication {
        service_uuid: SERVICE_UUID,
        service_name: SERVICE_NAME,
        characteristic_control,
        application_definition: Application {
            services: vec![Service {
                uuid: SERVICE_UUID,
                primary: true,
                characteristics: vec![Characteristic {
                    uuid: CHARACTERISTIC_UUID,
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
    }
}
