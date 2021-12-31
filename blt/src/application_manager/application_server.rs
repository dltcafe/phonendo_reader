use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

use crate::application_manager::{
    AdapterManager, ApplicationConfiguration, ApplicationFactory, GattApplication,
};

pub struct ApplicationServer {
    adapter_manager: AdapterManager,
    application_configuration: Option<ApplicationConfiguration>,
}

impl ApplicationServer {
    pub async fn start(application_factory: &dyn ApplicationFactory) -> Result<Self> {
        let mut application_server = ApplicationServer::new().await?;

        let adapter = application_server.adapter_manager.adapter();
        println!(
            "Advertising on Bluetooth adapter {} with address {}.",
            adapter.name(),
            adapter.address().await?
        );

        application_server
            .serve(application_factory.create())
            .await?;

        if let Some(configuration) = &application_server.application_configuration {
            println!(
                "GATT service '{}' ready. Press enter to quit.",
                configuration.service_name()
            )
        }

        Ok(application_server)
    }

    pub async fn new() -> Result<Self> {
        Ok(Self {
            adapter_manager: AdapterManager::new().await?,
            application_configuration: None,
        })
    }

    pub async fn serve(&mut self, gatt_application: GattApplication) -> Result<()> {
        self.application_configuration = Some(gatt_application.init(&self.adapter_manager).await?);
        Ok(())
    }

    pub async fn teardown(&mut self) {
        println!("Removing service and advertisement.");
        self.stop().await;
    }

    pub async fn stop(&mut self) {
        if let Some(application_configuration) = self.application_configuration.take() {
            application_configuration.drop();
            sleep(Duration::from_secs(1)).await;
        }
    }
}
