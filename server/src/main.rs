use anyhow::Result;
use blt::{applications::ping_pong, ApplicationServer};

#[tokio::main]
async fn main() -> Result<()> {
    ApplicationServer::start(ping_pong::gatt_application()).await?;
    Ok(())
}
