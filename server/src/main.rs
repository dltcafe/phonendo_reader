use anyhow::Result;
use blt::{applications::ping_pong::PingPong, ApplicationServer};

#[tokio::main]
async fn main() -> Result<()> {
    ApplicationServer::start(Box::new(PingPong::default())).await?;
    Ok(())
}
