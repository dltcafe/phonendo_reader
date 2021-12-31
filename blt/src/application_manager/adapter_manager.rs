use anyhow::Result;
use bluer::adv::{Advertisement, AdvertisementHandle};
use bluer::gatt::local::{Application, ApplicationHandle};
use bluer::Adapter;
use uuid::Uuid;

pub struct AdapterManager {
    adapter: Adapter,
}

impl AdapterManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            adapter: AdapterManager::connect_adapter().await?,
        })
    }

    async fn connect_adapter() -> Result<Adapter> {
        let session = bluer::Session::new().await?;
        let adapter_names = session.adapter_names().await?;
        let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
        let adapter = session.adapter(adapter_name)?;
        adapter.set_powered(true).await?;

        Ok(adapter)
    }

    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub async fn serve_gatt_application(
        &self,
        application: Application,
    ) -> Result<ApplicationHandle> {
        Ok(self.adapter.serve_gatt_application(application).await?)
    }

    pub async fn advertise_gatt_service(
        &self,
        service_uuid: Uuid,
        local_name: &str,
    ) -> Result<AdvertisementHandle> {
        Ok(self
            .adapter
            .advertise(Advertisement {
                service_uuids: vec![service_uuid].into_iter().collect(),
                discoverable: Some(true),
                local_name: Some(local_name.to_string()),
                ..Default::default()
            })
            .await?)
    }
}
