use anyhow::Result;

use crate::{AdapterManager, ApplicationConfiguration};
use bluer::{
    gatt::local::{Application, CharacteristicControl},
    Uuid,
};

pub struct GattApplication {
    service_uuid: Uuid,
    service_name: &'static str,
    application_definition: Application,
    characteristics_controls: Vec<CharacteristicControl>,
}

impl GattApplication {
    pub fn new(
        service_uuid: Uuid,
        service_name: &'static str,
        application_definition: Application,
        characteristics_controls: Vec<CharacteristicControl>,
    ) -> Self {
        Self {
            service_uuid,
            service_name,
            application_definition,
            characteristics_controls,
        }
    }

    pub fn service_uuid(&self) -> &Uuid {
        &self.service_uuid
    }

    pub fn service_name(&self) -> &'static str {
        self.service_name
    }

    pub fn application_definition(&self) -> &Application {
        &self.application_definition
    }

    pub fn characteristics_controls(&self) -> &Vec<CharacteristicControl> {
        &self.characteristics_controls
    }

    pub async fn init(self, adapter_manager: &AdapterManager) -> Result<ApplicationConfiguration> {
        let application_handle = adapter_manager
            .serve_gatt_application(self.application_definition)
            .await?;
        let advertisement_handle = adapter_manager
            .advertise_gatt_service(self.service_uuid, self.service_name)
            .await?;

        Ok(ApplicationConfiguration::new(
            self.service_name,
            self.characteristics_controls,
            application_handle,
            advertisement_handle,
        ))
    }
}
