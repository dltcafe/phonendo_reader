use crate::{AdapterManager, ApplicationDescriptor, BltApplication};
use anyhow::Result;
use bluer::{
    gatt::remote::{Characteristic, Service},
    AdapterEvent, Device,
};
use futures::{pin_mut, StreamExt};
use std::collections::HashMap;
use uuid::Uuid;

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

                    println!("\nDiscovered device {}.", device.address());
                    if !device.is_paired().await? {
                        println!("\tTrying to pair with it.");
                        device.pair().await?;
                        println!("\tPaired.");
                    }

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
                    match device.disconnect().await {
                        Ok(()) => println!("\tDevice disconnected."),
                        Err(error) => println!("\tDevice disconnection failed: {}.", &error),
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
        let uuids = device.uuids().await?.unwrap_or_default();
        if uuids.contains(self.application_descriptor.service_uuid()) {
            println!(
                "\tDevice provides service '{}'.",
                self.application_descriptor.service_name()
            );
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

            for service in device.services().await? {
                if service.uuid().await? == *self.application_descriptor.service_uuid() {
                    return Ok(Some(service));
                }
            }
        } else {
            println!("\tDevice doesn't provide our service.");
        }

        Ok(None)
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
