use crate::{AdapterManager, ApplicationDescriptor, BltApplication};
use anyhow::Result;
use bluer::{
    gatt::remote::{Characteristic, Service},
    AdapterEvent, Device,
};
use futures::{pin_mut, StreamExt};
use std::collections::HashMap;
use uuid::Uuid;

include!("../../resources/database.inc");

pub struct ApplicationClient {
    adapter_manager: AdapterManager,
    blt_application: Box<dyn BltApplication>,
    application_descriptor: ApplicationDescriptor,
    service: Option<Service>,
    characteristics: HashMap<Uuid, Characteristic>,
}

impl ApplicationClient {
    pub async fn start(blt_application: Box<dyn BltApplication>) -> Result<()> {
        let mut application_client = ApplicationClient::new(blt_application).await?;

        let adapter = application_client.adapter_manager.adapter();
        println!(
            "Discovering on Bluetooth adapter {} with address {}.",
            adapter.name(),
            adapter.address().await?
        );
        application_client.discover_service().await?;

        application_client.exercise_characteristics().await?;

        Ok(())
    }

    pub async fn new(blt_application: Box<dyn BltApplication>) -> Result<Self> {
        Ok(Self {
            adapter_manager: AdapterManager::new().await?,
            application_descriptor: blt_application.application_descriptor(),
            blt_application,
            service: None,
            characteristics: HashMap::new(),
        })
    }

    pub async fn discover_service(&mut self) -> Result<()> {
        let adapter = self.adapter_manager.adapter();
        let discover = adapter.discover_devices().await?;
        pin_mut!(discover);
        while let Some(event) = discover.next().await {
            match event {
                AdapterEvent::DeviceAdded(address) => {
                    let device = adapter.device(address)?;

                    println!(
                        "\nDiscovered device {}. [Name: {}. Alias: {}]",
                        device.address(),
                        device.name().await?.unwrap(),
                        device.alias().await.unwrap()
                    );

                    match self.find_application_service(&device).await {
                        Ok(Some(service)) => match self.find_characteristics(&service).await {
                            Ok(Some(characteristics)) => {
                                self.service = Some(service);
                                self.characteristics = characteristics;
                                break;
                            }
                            Ok(None) => (),
                            Err(error) => {
                                println!("\tDevice failed: {}.", &error);
                                let _ = adapter.remove_device(device.address()).await;
                            }
                        },
                        Ok(None) => (),
                        Err(error) => {
                            println!("\tDevice failed: {}.", &error);
                            let _ = adapter.remove_device(device.address()).await;
                        }
                    }

                    if device.is_connected().await? {
                        match device.disconnect().await {
                            Ok(()) => println!("\tDevice disconnected."),
                            Err(error) => println!("\tDevice disconnection failed: {}.", &error),
                        }
                    }
                }

                AdapterEvent::DeviceRemoved(address) => {
                    println!("Device removed {}.", address);
                }

                _ => (),
            }
        }

        println!("\nStopping discovery.");

        Ok(())
    }

    async fn find_application_service(&self, device: &Device) -> Result<Option<Service>> {
        self.device_prepare_for_discovering(device).await?;

        let uuids = device.uuids().await?.unwrap_or_default();
        if uuids.contains(self.application_descriptor.service_uuid()) {
            println!(
                "\tDevice provides service '{}'.",
                self.application_descriptor.service_name()
            );
            self.device_connect(device).await?;

            for service in device.services().await? {
                if service.uuid().await? == *self.application_descriptor.service_uuid() {
                    return Ok(Some(service));
                }
            }
        } else {
            println!("\tDevice doesn't provide our service.");
            match device.disconnect().await {
                Ok(()) => println!("\tDevice disconnected."),
                Err(error) => println!("\tDevice disconnection failed: {}.", &error),
            }
        }

        Ok(None)
    }

    async fn device_prepare_for_discovering(&self, device: &Device) -> Result<()> {
        if let Ok(need_pair) = self.device_need_pair(device).await {
            if need_pair {
                println!("\tDevice needs to be paired before scan it for provided services.");
                self.device_pair(device).await?
            }
        }

        Ok(())
    }

    async fn device_need_pair(&self, device: &Device) -> Result<bool> {
        Ok(DEVICES_TO_BE_PAIRED.contains(&device.alias().await.unwrap().as_str()))
    }

    async fn device_pair(&self, device: &Device) -> Result<()> {
        if !device.is_paired().await? {
            println!("\tPairing...");
            let mut retries = 5;
            loop {
                match device.pair().await {
                    Ok(()) => break,
                    Err(error) if retries > 0 => {
                        println!("\tPairing error: {}", &error);
                        retries -= 1;
                    }
                    Err(error) => return Err(anyhow::Error::new(error)),
                }
            }
            println!("\tPaired.");
        } else {
            println!("\tAlready paired.");
        }
        Ok(())
    }

    async fn device_connect(&self, device: &Device) -> Result<()> {
        if !device.is_connected().await? {
            println!("\tConnecting...");
            let mut retries = 2;
            loop {
                match device.connect().await {
                    Ok(()) => break,
                    Err(error) if retries > 0 => {
                        println!("\tConnect error: {}", &error);
                        retries -= 1;
                    }
                    Err(error) => return Err(anyhow::Error::new(error)),
                }
            }
            println!("\tConnected.");
        } else {
            println!("\tAlready connected.");
        }
        Ok(())
    }

    async fn find_characteristics(
        &self,
        service: &Service,
    ) -> Result<Option<HashMap<Uuid, Characteristic>>> {
        let mut characteristics = HashMap::new();
        for characteristic in service.characteristics().await? {
            let uuid = characteristic.uuid().await?;
            if self
                .application_descriptor
                .characteristics_uuids()
                .contains(&uuid)
            {
                characteristics.insert(uuid, characteristic);
            } else {
                println!("\tInvalid service characteristics (service provides an unknown characteristic).");
                characteristics.clear();
                return Ok(None);
            }
        }

        if characteristics.len() != self.application_descriptor.characteristics_uuids().len() {
            println!(
                "\tInvalid service characteristics (service doesn't support all characteristics)."
            );
            return Ok(None);
        }

        Ok(Some(characteristics))
    }

    async fn exercise_characteristics(&self) -> Result<()> {
        if self.service.is_some() {
            self.blt_application
                .exercise_characteristics(&self.characteristics)
                .await?;
        }

        Ok(())
    }
}
