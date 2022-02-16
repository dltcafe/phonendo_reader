use crate::{
    blt_application, ApplicationDescriptor, ApplicationHandler, BltApplication, GattApplication,
};
use anyhow::Result;
use async_trait::async_trait;
use bluer::gatt::local::{CharacteristicRead, CharacteristicWrite};
use bluer::gatt::remote::Characteristic;
use bluer::Uuid;
use chrono::{DateTime, Datelike, NaiveDateTime, ParseResult, Timelike};
use futures::FutureExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

include!("../../../resources/services/cts.inc");

const DIFF_IN_MINUTES_TO_FORCE_SYNC: i64 = 10;

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
            vec![Some(CharacteristicRead {
                read: true,
                fun: Box::new(|_| {
                    let current_local_time = chrono::Utc::now();
                    let current_local_time = date_time_to_vector(&current_local_time);
                    let value = Arc::new(Mutex::new(current_local_time));
                    async move {
                        let value = value.lock().await.clone();
                        Ok(value)
                    }
                    .boxed()
                }),
                ..Default::default()
            })],
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

    async fn serve(&self, application_handler: ApplicationHandler) -> Result<ApplicationHandler> {
        let mut receiver = blt_application::server_control_c_handler(&application_handler);
        'main_loop: loop {
            tokio::select! {
                _ = receiver.recv() => break 'main_loop,
            }
        }

        Ok(application_handler)
    }

    async fn exercise_characteristics(
        &self,
        characteristics: &HashMap<Uuid, Characteristic>,
    ) -> Result<()> {
        let characteristic = characteristics
            .get(&uuid::Uuid::try_from(CURRENT_TIME_CHARACTERISTIC).unwrap())
            .unwrap();

        let current_service_time = read_service_value(characteristic).await?;
        let current_local_time = chrono::Utc::now();
        println!("Current service time [UTC]: '{}'", current_service_time);
        println!("Current local time [UTC]: '{}'", current_local_time);
        let diff = (current_service_time.timestamp() - current_local_time.timestamp()).abs();
        if diff > 60 * DIFF_IN_MINUTES_TO_FORCE_SYNC {
            println!(
                "Difference is greater than {} minutes.",
                DIFF_IN_MINUTES_TO_FORCE_SYNC
            );
            println!("Changing the remote service time.");

            write_service_value(&current_local_time, characteristic).await?;
            println!(
                "Current service time [UTC]: '{}'",
                read_service_value(characteristic).await?
            );
        }

        Ok(())
    }
}

async fn read_service_value(characteristic: &Characteristic) -> Result<NaiveDateTime> {
    let current_service_time = characteristic.read().await?;
    Ok(vector_to_naive_date_time(&current_service_time).unwrap())
}

async fn write_service_value(
    time: &DateTime<chrono::Utc>,
    characteristic: &Characteristic,
) -> Result<()> {
    characteristic.write(&date_time_to_vector(time)).await?;
    Ok(())
}

fn date_time_to_vector(date_time: &DateTime<chrono::Utc>) -> Vec<u8> {
    vec![
        date_time.year() as u8,
        (date_time.year() >> 8) as u8,
        date_time.month() as u8,
        date_time.day() as u8,
        date_time.hour() as u8,
        date_time.minute() as u8,
        date_time.second() as u8,
        (date_time.weekday().num_days_from_monday() + 1) as u8,
        0x00,
    ]
}

fn vector_to_naive_date_time(time: &[u8]) -> ParseResult<NaiveDateTime> {
    let year = time[0] as u16 + ((time[1] as u16) << 8);
    let month = time[2];
    let day = time[3];
    let hour = time[4];
    let minute = time[5];
    let second = time[6];

    let date = format!(
        "{:#04}-{:#02}-{:#02} {:#02}:{:#02}:{:#02}",
        year, month, day, hour, minute, second
    );

    NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S")
}
