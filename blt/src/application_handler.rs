use crate::ApplicationDescriptor;
use bluer::{
    adv::AdvertisementHandle,
    gatt::local::{ApplicationHandle, CharacteristicControl},
};

pub struct ApplicationHandler {
    application_descriptor: ApplicationDescriptor,
    characteristics_controls: Vec<CharacteristicControl>,
    application_handle: ApplicationHandle,
    advertisement_handle: AdvertisementHandle,
}

impl ApplicationHandler {
    pub fn new(
        application_descriptor: ApplicationDescriptor,
        characteristics_controls: Vec<CharacteristicControl>,
        application_handle: ApplicationHandle,
        advertisement_handle: AdvertisementHandle,
    ) -> Self {
        Self {
            application_descriptor,
            characteristics_controls,
            application_handle,
            advertisement_handle,
        }
    }

    pub fn service_name(&self) -> &'static str {
        self.application_descriptor.service_name()
    }

    pub fn characteristics_controls(&self) -> &Vec<CharacteristicControl> {
        &self.characteristics_controls
    }

    pub fn pop_characteristic_control(&mut self) -> Option<CharacteristicControl> {
        self.characteristics_controls.pop()
    }

    pub fn application_handle(&self) -> &ApplicationHandle {
        &self.application_handle
    }

    pub fn advertisement_handle(&self) -> &AdvertisementHandle {
        &self.advertisement_handle
    }

    pub fn drop(self) {
        drop(self.application_handle);
        drop(self.advertisement_handle);
    }
}
