use crate::adder::Adder;
use crate::ping_pong::PingPong;

use crate::{ApplicationClient, ApplicationServer, BltApplication};

use crate::cts::CTS;
use anyhow::Result;
use std::env;
use std::str::FromStr;

const APP: &str = "APP";
const APP_MODE: &str = "APP_MODE";

#[derive(Debug, PartialEq)]
pub enum ApplicationMode {
    Client,
    Server,
}

impl FromStr for ApplicationMode {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let value = input.to_lowercase();
        match value.as_str() {
            "client" => Ok(ApplicationMode::Client),
            "server" => Ok(ApplicationMode::Server),
            _ => {
                println!("Unknown application mode '{}'", input);
                Err(())
            }
        }
    }
}

pub struct ApplicationFactory;

impl ApplicationFactory {
    pub async fn launch_application() -> Result<()> {
        if let Some(application) = ApplicationFactory::discover_application() {
            if let Some(application_mode) = ApplicationFactory::discover_mode() {
                match application_mode {
                    ApplicationMode::Client => ApplicationClient::start(application).await?,
                    ApplicationMode::Server => ApplicationServer::start(application).await?,
                };
            }
        }

        Ok(())
    }

    pub fn discover_mode() -> Option<ApplicationMode> {
        if let Some(application_mode) = ApplicationFactory::get_env_var(APP_MODE) {
            if let Ok(application_mode) = ApplicationMode::from_str(&application_mode) {
                return Some(application_mode);
            }
        }

        None
    }

    pub fn discover_application() -> Option<Box<dyn BltApplication>> {
        if let Some(app_name) = ApplicationFactory::get_env_var(APP) {
            ApplicationFactory::get_blt_application(&app_name)
        } else {
            None
        }
    }

    fn get_env_var(var: &str) -> Option<String> {
        if let Ok(result) = env::var(var) {
            Some(result)
        } else {
            println!("Environment var '{}' is not defined.", var);
            None
        }
    }

    fn get_blt_application(name: &str) -> Option<Box<dyn BltApplication>> {
        let value = name.to_lowercase();
        match value.as_str() {
            "ping_pong" => Some(Box::new(PingPong::default())),
            "adder" => Some(Box::new(Adder::default())),
            "cts" => Some(Box::new(CTS::default())),
            _ => {
                println!("Unknown application '{}'", name);
                None
            }
        }
    }
}
