use crate::blt_application::flush_notify_buffer;
use crate::{
    blt_application, ApplicationDescriptor, ApplicationHandler, BltApplication, GattApplication,
};
use anyhow::Result;
use async_trait::async_trait;
use bluer::gatt::local::CharacteristicWrite;
use bluer::gatt::remote::Characteristic;
use bluer::Uuid;
use std::collections::HashMap;

include!("../../../resources/services/heart_rate.inc");

pub struct HeartRate;

impl Default for HeartRate {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl BltApplication for HeartRate {
    fn application_descriptor(&self) -> ApplicationDescriptor {
        ApplicationDescriptor::new(
            uuid::Uuid::try_from(SERVICE).unwrap(),
            SERVICE_NAME,
            vec![uuid::Uuid::try_from(HEART_RATE_MEASUREMENT_CHARACTERISTIC).unwrap()],
            vec![ApplicationDescriptor::default_read()],
            vec![Some(CharacteristicWrite {
                write: false,
                ..Default::default()
            })],
            vec![ApplicationDescriptor::default_notify()],
        )
    }

    fn gatt_application(&self) -> GattApplication {
        GattApplication::from(self.application_descriptor())
    }

    async fn serve(&self, _application_handler: ApplicationHandler) -> Result<ApplicationHandler> {
        todo!("write")
    }

    async fn exercise_characteristics(
        &self,
        characteristics: &HashMap<Uuid, Characteristic>,
    ) -> Result<()> {
        let characteristic = characteristics
            .get(&uuid::Uuid::try_from(HEART_RATE_MEASUREMENT_CHARACTERISTIC).unwrap())
            .unwrap();

        let mut notify_io = characteristic.notify_io().await?;
        flush_notify_buffer(&mut notify_io).await?;
        println!("Flushed previous heart rate measurement notifications.\n");

        let mut receiver = blt_application::client_control_c_handler();
        'main_loop: loop {
            tokio::select! {
                _ = receiver.recv() => break 'main_loop,
                (aux_notify_io, result) = blt_application::read_from_characteristic(notify_io) => {
                    notify_io = aux_notify_io;
                    let buffer = result.expect("Read failed.");
                    println!(
                        "[{}] {:#3}.",
                        chrono::Utc::now().format("%F %T%.3f"),
                        vector_to_heart_rate(&buffer)
                    );
                },
            }
        }

        Ok(())
    }
}

fn vector_to_heart_rate(time: &[u8]) -> u16 {
    ((time[0] as u16) << 8) + time[1] as u16
}
