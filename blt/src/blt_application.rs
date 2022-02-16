use crate::{ApplicationDescriptor, ApplicationHandler, GattApplication};
use anyhow::Result;
use async_trait::async_trait;
use bluer::gatt::{
    remote::Characteristic,
    {CharacteristicReader, CharacteristicWriter},
};
use std::{collections::HashMap, io::Error, time::Duration};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::{io::AsyncReadExt, time::timeout};
use uuid::Uuid;

#[async_trait]
pub trait BltApplication {
    fn application_descriptor(&self) -> ApplicationDescriptor;
    fn gatt_application(&self) -> GattApplication;
    async fn serve(&self, application_handler: ApplicationHandler) -> Result<ApplicationHandler>;
    async fn exercise_characteristics(
        &self,
        characteristics: &HashMap<Uuid, Characteristic>,
    ) -> Result<()>;
}

pub async fn characteristic_io(
    uuid: &Uuid,
    characteristics: &HashMap<Uuid, Characteristic>,
) -> Result<(CharacteristicWriter, CharacteristicReader)> {
    if let Some(characteristic) = characteristics.get(uuid) {
        let write_io = characteristic.write_io().await?;
        println!("Obtained write IO. MTU {} bytes.", write_io.mtu());

        let mut notify_io = characteristic.notify_io().await?;
        println!("Obtained notification IO. MTU {} bytes.", notify_io.mtu());

        flush_notify_buffer(&mut notify_io).await?;
        println!("Flushed notification IO.");

        Ok((write_io, notify_io))
    } else {
        Err(anyhow::Error::msg(format!(
            "Characteristic '{}' not found.",
            uuid
        )))
    }
}

pub async fn flush_notify_buffer(notify_io: &mut CharacteristicReader) -> Result<()> {
    let mut buf = [0; 1024];
    while let Ok(Ok(_)) = timeout(Duration::from_secs(1), notify_io.read(&mut buf)).await {}
    Ok(())
}

pub async fn read_from_characteristic(
    mut characteristic_reader: CharacteristicReader,
) -> (CharacteristicReader, Result<Vec<u8>, Error>) {
    tokio::spawn(async move {
        let mut buffer = vec![0u8; 1024];
        let result = match characteristic_reader.read(&mut buffer).await {
            Ok(n) => Ok(buffer[..n].to_vec()),
            Err(error) => Err(error),
        };
        (characteristic_reader, result)
    })
    .await
    .unwrap()
}

pub fn server_control_c_handler(application_handler: &ApplicationHandler) -> Receiver<()> {
    println!(
        "GATT service '{}' ready. Press Ctrl+C to quit.",
        application_handler.service_name()
    );
    control_c_handler()
}

pub fn client_control_c_handler() -> Receiver<()> {
    println!("GATT service listening. Press Ctrl+C to quit.");
    control_c_handler()
}

fn control_c_handler() -> Receiver<()> {
    let (sender, receiver) = mpsc::channel(1);
    ctrlc::set_handler(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                println!(" Ctrl+C pressed.");
                sender.send(()).await.unwrap();
            });
    })
    .expect("Ctrl+C handler fails");

    receiver
}
