use crate::ping_pong::PingPong;
use crate::{ApplicationClient, ApplicationServer, BltApplication};

use anyhow::Result;
use std::env;

pub enum ApplicationMode {
    Client,
    Server,
}

pub async fn launch_application(application_mode: ApplicationMode) -> Result<()> {
    if let Some(application) = discover_application() {
        match application_mode {
            ApplicationMode::Client => ApplicationClient::start(application).await?,
            ApplicationMode::Server => ApplicationServer::start(application).await?,
        };
    }

    Ok(())
}

pub fn discover_application() -> Option<Box<dyn BltApplication>> {
    if let Ok(app_name) = env::var("APP") {
        get_blt_application(&app_name)
    } else {
        println!("Environment var 'APP' is not defined.");
        None
    }
}

pub fn get_blt_application(name: &str) -> Option<Box<dyn BltApplication>> {
    match name {
        "PingPong" => Some(Box::new(PingPong::default())),
        _ => {
            println!("Unknown application");
            None
        }
    }
}
