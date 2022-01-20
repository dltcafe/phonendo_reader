use anyhow::Result;

use crate::{AdapterManager, ApplicationDescriptor, ApplicationHandler};
use bluer::{
    gatt::local::{Application, CharacteristicControl},
    Uuid,
};

pub struct GattApplication {
    application_definition: Application,
    characteristics_controls: Vec<CharacteristicControl>,
    application_descriptor: ApplicationDescriptor,
}

impl GattApplication {
    pub fn new(
        application_definition: Application,
        characteristics_controls: Vec<CharacteristicControl>,
        application_descriptor: ApplicationDescriptor,
    ) -> Self {
        Self {
            application_definition,
            characteristics_controls,
            application_descriptor,
        }
    }

    pub fn service_uuid(&self) -> &Uuid {
        self.application_descriptor.service_uuid()
    }

    pub fn service_name(&self) -> &str {
        self.application_descriptor.service_name()
    }

    pub fn application_definition(&self) -> &Application {
        &self.application_definition
    }

    pub fn characteristics_controls(&self) -> &Vec<CharacteristicControl> {
        &self.characteristics_controls
    }

    pub async fn init(self, adapter_manager: &AdapterManager) -> Result<ApplicationHandler> {
        let application_handle = adapter_manager
            .serve_gatt_application(self.application_definition)
            .await?;
        let advertisement_handle = adapter_manager
            .advertise_gatt_service(
                *self.application_descriptor.service_uuid(),
                self.application_descriptor.service_name(),
            )
            .await?;

        Ok(ApplicationHandler::new(
            self.application_descriptor,
            self.characteristics_controls,
            application_handle,
            advertisement_handle,
        ))
    }
}
