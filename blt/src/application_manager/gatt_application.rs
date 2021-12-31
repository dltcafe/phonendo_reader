use anyhow::Result;

use crate::application_manager::{AdapterManager, ApplicationConfiguration};
use bluer::{
    gatt::local::{Application, CharacteristicControl},
    Uuid,
};

pub struct GattApplication {
    application_definition: Application,
    service_uuid: Uuid,
    characteristic_control: CharacteristicControl,
    service_name: &'static str,
}

impl GattApplication {
    pub fn new(
        application_definition: Application,
        service_uuid: Uuid,
        characteristic_control: CharacteristicControl,
        service_name: &'static str,
    ) -> Self {
        Self {
            application_definition,
            service_uuid,
            characteristic_control,
            service_name,
        }
    }

    pub fn application_definition(&self) -> &Application {
        &self.application_definition
    }

    pub fn service_uuid(&self) -> &Uuid {
        &self.service_uuid
    }

    pub fn characteristic_control(&self) -> &CharacteristicControl {
        &self.characteristic_control
    }

    pub fn service_name(&self) -> &'static str {
        self.service_name
    }

    pub async fn init(self, adapter_manager: &AdapterManager) -> Result<ApplicationConfiguration> {
        let application_handle = adapter_manager
            .serve_gatt_application(self.application_definition)
            .await?;
        let advertisement_handle = adapter_manager
            .advertise_gatt_service(self.service_uuid, self.service_name)
            .await?;

        Ok(ApplicationConfiguration::new(
            advertisement_handle,
            application_handle,
            self.characteristic_control,
            self.service_name,
        ))
    }
}
