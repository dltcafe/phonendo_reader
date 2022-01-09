use crate::{AdapterManager, ApplicationConfiguration, GattApplication};
use anyhow::Result;
use bluer::gatt::{local::CharacteristicControlEvent, CharacteristicReader, CharacteristicWriter};
use futures::{future, pin_mut, StreamExt};
use std::time::Duration;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
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
        if let Some(mut configuration) = application_server.application_configuration.take() {
            println!(
                "GATT service '{}' ready. Press enter to quit.",
                configuration.service_name()
            );

            let mut lines = BufReader::new(tokio::io::stdin()).lines();

            let mut read_buffer = Vec::new();
            let mut characteristic_reader: Option<CharacteristicReader> = None;
            let mut characteristic_writer: Option<CharacteristicWriter> = None;

            let characteristic_control = configuration.pop_characteristic_control().unwrap();
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
