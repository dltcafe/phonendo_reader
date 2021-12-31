mod ping_pong_application;

use anyhow::Result;
use blt::ApplicationServer;

#[tokio::main]
async fn main() -> Result<()> {
    ApplicationServer::start(ping_pong_application::gatt_application()).await?;
    Ok(())
}
