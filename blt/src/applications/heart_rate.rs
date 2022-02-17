use crate::blt_application::flush_notify_buffer;
use crate::{
    blt_application, ApplicationDescriptor, ApplicationHandler, BltApplication, GattApplication,
};
use anyhow::Result;
use async_trait::async_trait;
use bluer::gatt::local::{CharacteristicControlEvent, CharacteristicRead, CharacteristicWrite};
use bluer::gatt::remote::Characteristic;
use bluer::gatt::CharacteristicWriter;
use bluer::Uuid;
use futures::{pin_mut, FutureExt, StreamExt};
use rand::Rng;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::Once;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time::interval;

include!("../../../resources/services/heart_rate.inc");

const INITIAL_HEART_RATE_MEASURE: u16 = 80;
const NOTIFICATION_INTERVAL: u64 = 7;
const MAX_HEART_RATE: u16 = 250;
const MIN_HEART_RATE: u16 = 60;

#[derive(Default)]
struct ApplicationState {
    pub state: Mutex<Vec<u8>>,
}

fn application_state() -> &'static ApplicationState {
    static mut APPLICATION_STATE: MaybeUninit<ApplicationState> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let application_state = ApplicationState {
                state: Mutex::new(Vec::new()),
            };
            APPLICATION_STATE.write(application_state);
        });
        APPLICATION_STATE.assume_init_ref()
    }
}

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
            vec![Some(CharacteristicRead {
                read: true,
                fun: Box::new(|_| {
                    async move { Ok(application_state().state.lock().await.clone()) }.boxed()
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

    async fn serve(
        &self,
        mut application_handler: ApplicationHandler,
    ) -> Result<ApplicationHandler> {
        let mut receiver = blt_application::server_control_c_handler(&application_handler);

        let mut characteristic_writer: Option<CharacteristicWriter> = None;
        let characteristic_control = application_handler.pop_characteristic_control().unwrap();
        let mut interval = interval(Duration::from_secs(NOTIFICATION_INTERVAL));

        pin_mut!(characteristic_control);

        {
            let mut state = application_state().state.lock().await;
            *state = heart_rate_to_vector(&INITIAL_HEART_RATE_MEASURE);
        }

        'main_loop: loop {
            tokio::select! {
                _ = receiver.recv() => break 'main_loop,
                evt = characteristic_control.next() => {
                    match evt {
                        Some(CharacteristicControlEvent::Notify(notifier)) => {
                            characteristic_writer = Some(notifier);
                        },
                        _ => break,
                    }
                },
                _ = interval.tick() => {
                    let mut state = application_state().state.lock().await;
                    let heart_rate = generate_random_heart_rate_measure(&vector_to_heart_rate(&state));
                    *state = heart_rate_to_vector(&heart_rate);
                    println!("Generated new random value: {:#3}.", heart_rate);
                    if let Some(writer) = characteristic_writer.as_mut() {
                        if let Err(err) = writer.write(&state).await {
                            println!("Notification stream error: {}.", &err);
                            characteristic_writer = None;
                        }
                    }
                }
            }
        }

        Ok(application_handler)
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

fn vector_to_heart_rate(vector: &[u8]) -> u16 {
    ((vector[0] as u16) << 8) + vector[1] as u16
}

fn heart_rate_to_vector(heart_rate: &u16) -> Vec<u8> {
    vec![(heart_rate >> 8) as u8, *heart_rate as u8]
}

fn generate_random_heart_rate_measure(previous_value: &u16) -> u16 {
    let mut rnd = rand::thread_rng();
    let factor: f32 = *previous_value as f32 * rnd.gen_range(0.0..0.05);
    let direction = rnd.gen_range(-1..2);
    let change = (factor * direction as f32) as i16;
    let value = (*previous_value as i16 + change) as u16;
    value.min(MAX_HEART_RATE).max(MIN_HEART_RATE) as u16
}
