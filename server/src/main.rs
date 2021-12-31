mod ping_pong_application;

use anyhow::Result;

use crate::ping_pong_application::PingPongApplication;
use blt::application_manager::ApplicationServer;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<()> {
    let application_factory = PingPongApplication::new();
    let mut application_server = ApplicationServer::start(&application_factory).await?;

    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    loop {
        tokio::select! {
            _ = lines.next_line() => break,
        }
    }

    application_server.teardown().await;
    Ok(())
}
