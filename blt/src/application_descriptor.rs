use crate::GattApplication;
use bluer::gatt::local::{
    Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod,
    CharacteristicWrite, CharacteristicWriteMethod, Service,
};
use bluer::Uuid;

pub struct ApplicationDescriptor {
    service_uuid: Uuid,
    service_name: &'static str,
    characteristics_uuids: Vec<Uuid>,
}

impl ApplicationDescriptor {
    pub fn new(
        service_uuid: Uuid,
        service_name: &'static str,
        characteristics_uuids: Vec<Uuid>,
    ) -> Self {
        Self {
            service_uuid,
            service_name,
            characteristics_uuids,
        }
    }

    pub fn service_uuid(&self) -> &Uuid {
        &self.service_uuid
    }

    pub fn service_name(&self) -> &'static str {
        self.service_name
    }

    pub fn characteristics_uuids(&self) -> &Vec<Uuid> {
        &self.characteristics_uuids
    }
}

impl From<ApplicationDescriptor> for GattApplication {
    fn from(application_descriptor: ApplicationDescriptor) -> Self {
        let mut characteristics_controls_handles = Vec::new();
        let mut characteristics_controls = Vec::new();
        for _ in 0..application_descriptor.characteristics_uuids.len() {
            let (characteristic_control, characteristic_control_handle) =
                bluer::gatt::local::characteristic_control();
            characteristics_controls_handles.push(characteristic_control_handle);
            characteristics_controls.push(characteristic_control);
        }
        characteristics_controls_handles.reverse();

        GattApplication::new(
            Application {
                services: vec![Service {
                    uuid: application_descriptor.service_uuid,
                    primary: true,
                    characteristics: application_descriptor
                        .characteristics_uuids
                        .iter()
                        .map(|uuid| Characteristic {
                            uuid: *uuid,
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
                            control_handle: characteristics_controls_handles.pop().unwrap(),
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            characteristics_controls,
            application_descriptor,
        )
    }
}
