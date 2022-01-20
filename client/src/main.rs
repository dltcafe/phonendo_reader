use anyhow::Result;
use blt::{applications::ping_pong::PingPong, ApplicationClient};

#[tokio::main]
async fn main() -> Result<()> {
    ApplicationClient::start(Box::new(PingPong::default())).await?;

    Ok(())
}
