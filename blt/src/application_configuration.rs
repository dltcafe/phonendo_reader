use bluer::{
    adv::AdvertisementHandle,
    gatt::local::{ApplicationHandle, CharacteristicControl},
};

pub struct ApplicationConfiguration {
    service_name: &'static str,
    characteristics_controls: Vec<CharacteristicControl>,
    application_handle: ApplicationHandle,
    advertisement_handle: AdvertisementHandle,
}

impl ApplicationConfiguration {
    pub fn new(
        service_name: &'static str,
        characteristics_controls: Vec<CharacteristicControl>,
        application_handle: ApplicationHandle,
        advertisement_handle: AdvertisementHandle,
    ) -> Self {
        Self {
            service_name,
            characteristics_controls,
            application_handle,
            advertisement_handle,
        }
    }

    pub fn service_name(&self) -> &'static str {
        self.service_name
    }

    pub fn characteristics_controls(&self) -> &Vec<CharacteristicControl> {
        &self.characteristics_controls
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
