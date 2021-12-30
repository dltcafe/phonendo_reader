mod ping_pong_application;
mod server_manager;

use anyhow::Result;

use crate::server_manager::ServerManager;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<()> {
    let mut server_manager = ServerManager::new().await?;

    let adapter = server_manager.adapter();
    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        adapter.name(),
        adapter.address().await?
    );

    server_manager
        .init_application(ping_pong_application::create_application_definition())
        .await?;

    let application_configuration = server_manager.application_configuration().unwrap();
    println!(
        "GATT service '{}' ready. Press enter to quit.",
        application_configuration.service_name
    );

    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    loop {
        tokio::select! {
            _ = lines.next_line() => break,
        }
    }

    println!("Removing service and advertisement");
    server_manager.stop().await;

    Ok(())
}
