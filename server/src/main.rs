use anyhow::Result;
use blt::{application_factory, ApplicationServer};

use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(app_name) = env::var("APP") {
        if let Some(application) = application_factory::get_blt_application(&app_name) {
            ApplicationServer::start(application).await?;
        } else {
            println!("Unknown application.");
        }
    } else {
        println!("Environment var 'APP' is not defined.");
    }

    Ok(())
}
