use crate::{ApplicationDescriptor, ApplicationHandler, BltApplication, GattApplication};
use anyhow::Result;
use async_trait::async_trait;
use bluer::{gatt::remote::Characteristic, Uuid};
use std::collections::HashMap;

include!("../../../resources/services/cts.inc");

pub struct CTS;

impl Default for CTS {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl BltApplication for CTS {
    fn application_descriptor(&self) -> ApplicationDescriptor {
        ApplicationDescriptor::new(
            uuid::Uuid::try_from(SERVICE).unwrap(),
            SERVICE_NAME,
            vec![uuid::Uuid::try_from(CURRENT_TIME_CHARACTERISTIC).unwrap()],
        )
    }

    fn gatt_application(&self) -> GattApplication {
        GattApplication::from(self.application_descriptor())
    }

    async fn serve(
        &self,
        mut _application_handler: ApplicationHandler,
    ) -> Result<ApplicationHandler> {
        todo!("serve");
    }

    async fn exercise_characteristics(
        &self,
        _characteristics: &HashMap<Uuid, Characteristic>,
    ) -> Result<()> {
        todo!("Exercise characteristic");
    }
}
