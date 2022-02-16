use crate::GattApplication;
use bluer::gatt::local::{
    Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod,
    CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod, Service,
};
use bluer::Uuid;

pub struct ApplicationDescriptor {
    service_uuid: Uuid,
    service_name: &'static str,
    characteristics_uuids: Vec<Uuid>,
    read_functions: Vec<Option<CharacteristicRead>>,
    write_functions: Vec<Option<CharacteristicWrite>>,
    notify_functions: Vec<Option<CharacteristicNotify>>,
}

impl ApplicationDescriptor {
    pub fn new(
        service_uuid: Uuid,
        service_name: &'static str,
        characteristics_uuids: Vec<Uuid>,
        read_functions: Vec<Option<CharacteristicRead>>,
        write_functions: Vec<Option<CharacteristicWrite>>,
        notify_functions: Vec<Option<CharacteristicNotify>>,
    ) -> Self {
        Self {
            service_uuid,
            service_name,
            characteristics_uuids,
            read_functions,
            write_functions,
            notify_functions,
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

    pub fn default_reade() -> Option<CharacteristicRead> {
        None
    }

    pub fn default_write() -> Option<CharacteristicWrite> {
        Some(CharacteristicWrite {
            write_without_response: true,
            method: CharacteristicWriteMethod::Io,
            ..Default::default()
        })
    }

    pub fn default_notify() -> Option<CharacteristicNotify> {
        Some(CharacteristicNotify {
            notify: true,
            method: CharacteristicNotifyMethod::Io,
            ..Default::default()
        })
    }

    pub fn default_descriptor(
        service_uuid: Uuid,
        service_name: &'static str,
        characteristics_uuids: Vec<Uuid>,
    ) -> ApplicationDescriptor {
        let mut read_functions = Vec::new();
        let mut write_functions = Vec::new();
        let mut notify_functions = Vec::new();

        for _ in 0..characteristics_uuids.len() {
            read_functions.push(None);
            write_functions.push(None);
            notify_functions.push(None);
        }

        ApplicationDescriptor::new(
            service_uuid,
            service_name,
            characteristics_uuids,
            read_functions,
            write_functions,
            notify_functions,
        )
    }
}

impl From<ApplicationDescriptor> for GattApplication {
    fn from(mut application_descriptor: ApplicationDescriptor) -> Self {
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
                            read: application_descriptor.read_functions.pop().unwrap(),
                            write: application_descriptor.write_functions.pop().unwrap(),
                            notify: application_descriptor.notify_functions.pop().unwrap(),
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
