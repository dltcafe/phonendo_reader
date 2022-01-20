use crate::{
    blt_application, ApplicationDescriptor, ApplicationHandler, BltApplication, GattApplication,
};
use anyhow::Result;
use async_trait::async_trait;
use bluer::gatt::remote::Characteristic;
use bluer::gatt::{local::CharacteristicControlEvent, CharacteristicReader, CharacteristicWriter};
use futures::{future, pin_mut, StreamExt};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use uuid::Uuid;

include!("../../../resources/services/ping_pong.inc");

pub struct PingPong;

impl Default for PingPong {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl BltApplication for PingPong {
    fn application_descriptor(&self) -> ApplicationDescriptor {
        ApplicationDescriptor::new(SERVICE_UUID, SERVICE_NAME, vec![CHARACTERISTIC_UUID])
    }

    fn gatt_application(&self) -> GattApplication {
        GattApplication::from(self.application_descriptor())
    }

    async fn serve(&self, application_handler: &mut ApplicationHandler) -> Result<()> {
        println!(
            "GATT service '{}' ready. Press enter to quit.",
            application_handler.service_name()
        );

        let mut lines = BufReader::new(tokio::io::stdin()).lines();

        let mut read_buffer = Vec::new();
        let mut characteristic_reader: Option<CharacteristicReader> = None;
        let mut characteristic_writer: Option<CharacteristicWriter> = None;

        let characteristic_control = application_handler.pop_characteristic_control().unwrap();
        pin_mut!(characteristic_control);

        'main_loop: loop {
            tokio::select! {
                _ = lines.next_line() => break 'main_loop,
                evt = characteristic_control.next() => {
                    match evt {
                        Some(CharacteristicControlEvent::Write(req)) => {
                            read_buffer = vec![0; req.mtu()];
                            characteristic_reader = Some(req.accept()?);
                        },
                        Some(CharacteristicControlEvent::Notify(notifier)) => {
                            characteristic_writer = Some(notifier);
                        },
                        None => break,
                    }
                },
                read_res = async {
                    match &mut characteristic_reader {
                        Some(reader) if characteristic_writer.is_some() => reader.read(&mut read_buffer).await,
                        _ => future::pending().await,
                    }
                } => {
                    match read_res {
                        Ok(0) => {
                            characteristic_reader = None;
                        },
                        Ok(n) => {
                            let value = read_buffer[..n].to_vec();
                            let string = String::from_utf8_lossy(&value);
                            let output  = match string.as_ref() {
                                "ping" => "pong",
                                "exit" => "bye!",
                                _ => "",
                            }.as_bytes().to_vec();

                            if let Err(err) = characteristic_writer.as_mut().unwrap().write_all(&output).await {
                                println!("Write failed: {}", &err);
                                characteristic_writer = None;
                            }

                            if string == "exit" {
                                break 'main_loop;
                            }

                        },
                        Err(err) => {
                            println!("Read stream error: {}", &err);
                            characteristic_reader = None;
                        },
                    }
                },
            }
        }

        Ok(())
    }

    async fn exercise_characteristics(
        &self,
        characteristics: &HashMap<Uuid, Characteristic>,
    ) -> Result<()> {
        for uuid in characteristics.keys() {
            let (mut write_io, notify_io) =
                blt_application::characteristic_io(uuid, characteristics).await?;

            let data: Vec<u8> = "ping".as_bytes().to_vec();

            write_io.write_all(&data).await.expect("Write failed.");
            let (_notify_io, result) =
                blt_application::read_from_characteristic(notify_io, data.len()).await;

            let buffer = result.expect("Read failed.");
            println!("Server says {:?}", String::from_utf8_lossy(&buffer));
        }

        Ok(())
    }
}
