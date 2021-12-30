use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::local::{ApplicationHandle, CharacteristicControl},
    Adapter,
};

use anyhow::Result;
use bluer::gatt::local::Application;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub struct ApplicationConfiguration {
    pub advertising_handler: AdvertisementHandle,
    pub application_handle: ApplicationHandle,
    pub characteristic_control: CharacteristicControl,
    pub service_name: &'static str,
}

pub struct GattApplication {
    pub application_definition: Application,
    pub service_uuid: Uuid,
    pub characteristic_control: CharacteristicControl,
    pub service_name: &'static str,
}

pub struct ServerManager {
    adapter: Adapter,
    application_configuration: Option<ApplicationConfiguration>,
}

impl ServerManager {
    pub async fn new() -> Result<Self> {
        let adapter = ServerManager::connect_adapter().await?;

        Ok(Self {
            adapter,
            application_configuration: None,
        })
    }

    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub fn application_configuration(&self) -> Option<&ApplicationConfiguration> {
        if let Some(application_configuration) = &self.application_configuration {
            Some(application_configuration)
        } else {
            None
        }
    }

    async fn connect_adapter() -> Result<Adapter> {
        let session = bluer::Session::new().await?;
        let adapter_names = session.adapter_names().await?;
        let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
        let adapter = session.adapter(adapter_name)?;
        adapter.set_powered(true).await?;

        Ok(adapter)
    }

    pub async fn init_application(&mut self, gatt_application: GattApplication) -> Result<()> {
        let application_handle = self
            .adapter
            .serve_gatt_application(gatt_application.application_definition)
            .await?;
        let advertising_handler = advertise_gatt_service(
            &self.adapter,
            vec![gatt_application.service_uuid],
            gatt_application.service_name.to_string(),
        )
        .await?;

        self.application_configuration = Some(ApplicationConfiguration {
            advertising_handler,
            application_handle,
            characteristic_control: gatt_application.characteristic_control,
            service_name: gatt_application.service_name,
        });

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(application_configuration) = self.application_configuration.take() {
            drop(application_configuration.application_handle);
            drop(application_configuration.advertising_handler);
            sleep(Duration::from_secs(1)).await;
        }
    }
}

pub async fn advertise_gatt_service(
    adapter: &Adapter,
    service_uuids: Vec<Uuid>,
    local_name: String,
) -> Result<AdvertisementHandle> {
    let le_advertisement = Advertisement {
        service_uuids: service_uuids.into_iter().collect(),
        discoverable: Some(true),
        local_name: Some(local_name),
        ..Default::default()
    };

    Ok(adapter.advertise(le_advertisement).await?)
}
