use bluer::{
    adv::AdvertisementHandle,
    gatt::local::{ApplicationHandle, CharacteristicControl},
};

pub struct ApplicationConfiguration {
    advertisement_handle: AdvertisementHandle,
    application_handle: ApplicationHandle,
    characteristic_control: CharacteristicControl,
    service_name: &'static str,
}

impl ApplicationConfiguration {
    pub fn new(
        advertisement_handle: AdvertisementHandle,
        application_handle: ApplicationHandle,
        characteristic_control: CharacteristicControl,
        service_name: &'static str,
    ) -> Self {
        Self {
            advertisement_handle,
            application_handle,
            characteristic_control,
            service_name,
        }
    }

    pub fn advertisement_handle(&self) -> &AdvertisementHandle {
        &self.advertisement_handle
    }

    pub fn application_handle(&self) -> &ApplicationHandle {
        &self.application_handle
    }

    pub fn characteristic_control(&self) -> &CharacteristicControl {
        &self.characteristic_control
    }

    pub fn service_name(&self) -> &'static str {
        self.service_name
    }

    pub fn drop(self) {
        drop(self.application_handle);
        drop(self.advertisement_handle);
    }
}
