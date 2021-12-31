use crate::{AdapterManager, ApplicationConfiguration, GattApplication};
use anyhow::Result;
use std::time::Duration;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};

pub struct ApplicationServer {
    adapter_manager: AdapterManager,
    application_configuration: Option<ApplicationConfiguration>,
}

impl ApplicationServer {
    pub async fn start(gatt_application: GattApplication) -> Result<()> {
        let mut application_server = ApplicationServer::new().await?;

        let adapter = application_server.adapter_manager.adapter();
        println!(
            "Advertising on Bluetooth adapter {} with address {}.",
            adapter.name(),
            adapter.address().await?
        );

        application_server.serve(gatt_application).await?;
        if let Some(configuration) = &application_server.application_configuration {
            println!(
                "GATT service '{}' ready. Press enter to quit.",
                configuration.service_name()
            );

            let mut lines = BufReader::new(tokio::io::stdin()).lines();
            loop {
                tokio::select! {
                    _ = lines.next_line() => break,
                }
            }

            application_server.teardown().await;
        }

        Ok(())
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
