use crate::{
    blt_application, ApplicationDescriptor, ApplicationHandler, BltApplication, GattApplication,
};
use anyhow::Result;
use async_trait::async_trait;
use bluer::gatt::remote::Characteristic;
use bluer::gatt::{local::CharacteristicControlEvent, CharacteristicReader, CharacteristicWriter};
use bluer::Uuid;
use futures::{future, pin_mut, StreamExt};
use rand::Rng;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

include!("../../../resources/services/adder.inc");

pub struct Adder;

impl Default for Adder {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl BltApplication for Adder {
    fn application_descriptor(&self) -> ApplicationDescriptor {
        ApplicationDescriptor::default_descriptor(
            SERVICE_UUID,
            SERVICE_NAME,
            vec![CHARACTERISTIC_UUID],
        )
    }

    fn gatt_application(&self) -> GattApplication {
        GattApplication::from(self.application_descriptor())
    }

    async fn serve(
        &self,
        mut application_handler: ApplicationHandler,
    ) -> Result<ApplicationHandler> {
        let mut read_buffer = Vec::new();
        let mut characteristic_reader: Option<CharacteristicReader> = None;
        let mut characteristic_writer: Option<CharacteristicWriter> = None;

        let characteristic_control = application_handler.pop_characteristic_control().unwrap();
        pin_mut!(characteristic_control);

        let mut receiver = blt_application::server_control_c_handler(&application_handler);

        'main_loop: loop {
            tokio::select! {
                _ = receiver.recv() => break 'main_loop,
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
                                "exit" => "stopping emulator".to_string(),
                                value => sum(value),
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

        Ok(application_handler)
    }

    async fn exercise_characteristics(
        &self,
        characteristics: &HashMap<Uuid, Characteristic>,
    ) -> Result<()> {
        for uuid in characteristics.keys() {
            let (mut write_io, mut notify_io) =
                blt_application::characteristic_io(uuid, characteristics).await?;

            for message in generate_random_entries() {
                let data: Vec<u8> = message.as_bytes().to_vec();

                println!("\n>> Command:  {:?}.", message);
                write_io.write_all(&data).await.expect("Write failed.");

                let (aux_notify_io, result) =
                    blt_application::read_from_characteristic(notify_io).await;

                notify_io = aux_notify_io;

                let buffer = result.expect("Read failed.");
                println!(
                    "<< Response: {:?}.",
                    String::from_utf8_lossy(&buffer).trim()
                );
            }
        }

        Ok(())
    }
}

fn sum(str: &str) -> String {
    let mut result = 0;

    for value in str.split(' ').collect::<Vec<&str>>() {
        if let Ok(n) = value.parse::<i32>() {
            result += n;
        } else {
            return format!("Invalid number '{}'", value);
        }
    }

    result.to_string()
}

fn generate_random_entries() -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..rng.gen_range(1..10) {
        // entries
        entries.push(
            (0..rng.gen_range(1..5)) // values per entry
                .map(|_| rng.gen_range(0..11).to_string()) // value
                .collect::<Vec<String>>()
                .join(" "),
        );
    }
    entries.push("1 a".to_string()); // Invalid entry
    entries.push("exit".to_string()); // Stop entry

    entries
}
