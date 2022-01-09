use crate::{AdapterManager, ApplicationDescriptor};
use anyhow::Result;
use bluer::gatt::{CharacteristicReader, CharacteristicWriter};
use bluer::{
    gatt::remote::{Characteristic, Service},
    AdapterEvent, Device,
};
use futures::{pin_mut, StreamExt};
use std::io::Error;
use std::{collections::HashMap, time::Duration};
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncReadExt, time::timeout};
use uuid::Uuid;

pub struct ApplicationClient {
    adapter_manager: AdapterManager,
    application_descriptor: ApplicationDescriptor,
    service: Option<Service>,
    characteristics: HashMap<Uuid, Characteristic>,
}

impl ApplicationClient {
    pub async fn start(application_descriptor: ApplicationDescriptor) -> Result<()> {
        let mut application_client = ApplicationClient::new(application_descriptor).await?;

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

    pub async fn new(application_descriptor: ApplicationDescriptor) -> Result<Self> {
        Ok(Self {
            adapter_manager: AdapterManager::new().await?,
            application_descriptor,
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
            for uuid in self.characteristics.keys() {
                let (mut write_io, notify_io) = self.characteristic_io(uuid).await?;

                let data: Vec<u8> = "ping".as_bytes().to_vec();

                write_io.write_all(&data).await.expect("Write failed.");
                let (_notify_io, result) =
                    ApplicationClient::read_from_characteristic(notify_io, data.len()).await;

                let buffer = result.expect("Read failed.");
                println!("Server says {:?}", String::from_utf8_lossy(&buffer));
            }
        }

        Ok(())
    }

    async fn characteristic_io(
        &self,
        uuid: &Uuid,
    ) -> Result<(CharacteristicWriter, CharacteristicReader)> {
        if let Some(characteristic) = self.characteristics.get(uuid) {
            let write_io = characteristic.write_io().await?;
            println!("Obtained write IO. MTU {} bytes.", write_io.mtu());

            let mut notify_io = characteristic.notify_io().await?;
            println!("Obtained notification IO. MTU {} bytes.", notify_io.mtu());

            ApplicationClient::flush_notify_buffer(&mut notify_io).await?;
            println!("Flushed notification IO.");

            Ok((write_io, notify_io))
        } else {
            Err(anyhow::Error::msg(format!(
                "Characteristic '{}' not found.",
                uuid
            )))
        }
    }

    async fn flush_notify_buffer(notify_io: &mut CharacteristicReader) -> Result<()> {
        let mut buf = [0; 1024];
        while let Ok(Ok(_)) = timeout(Duration::from_secs(1), notify_io.read(&mut buf)).await {}
        Ok(())
    }

    async fn read_from_characteristic(
        mut characteristic_reader: CharacteristicReader,
        len: usize,
    ) -> (CharacteristicReader, Result<Vec<u8>, Error>) {
        tokio::spawn(async move {
            let mut buffer = vec![0u8; len];
            let result = match characteristic_reader.read_exact(&mut buffer).await {
                Ok(_) => Ok(buffer),
                Err(error) => Err(error),
            };
            (characteristic_reader, result)
        })
        .await
        .unwrap()
    }
}
