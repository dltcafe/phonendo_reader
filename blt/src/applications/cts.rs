use crate::{ApplicationDescriptor, ApplicationHandler, BltApplication, GattApplication};
use anyhow::Result;
use async_trait::async_trait;
use bluer::gatt::remote::Characteristic;
use bluer::Uuid;
use chrono::{Datelike, Timelike};
use std::collections::HashMap;

include!("../../../resources/services/cts.inc");

pub struct CTS;

impl Default for CTS {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl BltApplication for CTS {
    fn application_descriptor(&self) -> ApplicationDescriptor {
        ApplicationDescriptor::new(
            uuid::Uuid::try_from(SERVICE).unwrap(),
            SERVICE_NAME,
            vec![uuid::Uuid::try_from(CURRENT_TIME_CHARACTERISTIC).unwrap()],
        )
    }

    fn gatt_application(&self) -> GattApplication {
        GattApplication::from(self.application_descriptor())
    }

    async fn serve(
        &self,
        mut _application_handler: ApplicationHandler,
    ) -> Result<ApplicationHandler> {
        todo!("serve");
    }

    async fn exercise_characteristics(
        &self,
        characteristics: &HashMap<Uuid, Characteristic>,
    ) -> Result<()> {
        let characteristic = characteristics
            .get(&uuid::Uuid::try_from(CURRENT_TIME_CHARACTERISTIC).unwrap())
            .unwrap();

        let vector_to_naive_date_time = |time: Vec<u8>| {
            let result = chrono::NaiveDateTime::parse_from_str(
                format!(
                    "{:#04}-{:#02}-{:#02} {:#02}:{:#02}:{:#02}",
                    time[0] as u16 + ((time[1] as u16) << 8),
                    time[2],
                    time[3],
                    time[4],
                    time[5],
                    time[6]
                )
                .as_str(),
                "%Y-%m-%d %H:%M:%S",
            );
            result
        };

        let current_service_time = characteristic.read().await?;
        let current_service_time = vector_to_naive_date_time(current_service_time).unwrap();
        let current_local_time = chrono::Utc::now();
        println!("Current service time [UTC]: '{}'", current_service_time);
        println!("Current local time [UTC]: '{}'", current_local_time);
        let difference = (current_service_time.timestamp() - current_local_time.timestamp()).abs();
        if difference > 60 * 10 {
            println!("Difference is greater than thirty minutes.");
            println!("Changing the remote service time.");
            characteristic
                .write(&[
                    current_local_time.year() as u8,        // lsb
                    (current_local_time.year() >> 8) as u8, // msb
                    current_local_time.month() as u8,
                    current_local_time.day() as u8,
                    current_local_time.hour() as u8,
                    current_local_time.minute() as u8,
                    current_local_time.second() as u8,
                    (current_local_time.weekday().num_days_from_monday() + 1) as u8,
                    0x00,
                ])
                .await?;

            let current_service_time = characteristic.read().await?;
            let current_service_time = vector_to_naive_date_time(current_service_time).unwrap();
            println!("Current service time [UTC]: '{}'", current_service_time);
        }

        Ok(())
    }
}
