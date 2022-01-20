use crate::{AdapterManager, ApplicationHandler, BltApplication};
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

pub struct ApplicationServer {
    blt_application: Box<dyn BltApplication>,
    adapter_manager: AdapterManager,
}

impl ApplicationServer {
    pub async fn start(blt_application: Box<dyn BltApplication>) -> Result<()> {
        let mut application_server = ApplicationServer::new(blt_application).await?;

        let adapter = application_server.adapter_manager.adapter();
        println!(
            "Advertising on Bluetooth adapter {} with address {}.",
            adapter.name(),
            adapter.address().await?
        );

        application_server.serve().await?;

        Ok(())
    }

    pub async fn new(blt_application: Box<dyn BltApplication>) -> Result<Self> {
        Ok(Self {
            blt_application,
            adapter_manager: AdapterManager::new().await?,
        })
    }

    pub async fn serve(&mut self) -> Result<()> {
        let gatt_application = self.blt_application.gatt_application();
        let mut application_handler = gatt_application.init(&self.adapter_manager).await?;
        self.blt_application.serve(&mut application_handler).await?;
        ApplicationServer::teardown(application_handler).await;
        Ok(())
    }

    pub async fn teardown(application_handler: ApplicationHandler) {
        println!("Removing service and advertisement.");
        ApplicationServer::stop(application_handler).await;
    }

    pub async fn stop(application_handler: ApplicationHandler) {
        application_handler.drop();
        sleep(Duration::from_secs(1)).await;
    }
}
