use anyhow::Result;
use blt::{applications::ping_pong, ApplicationClient};

#[tokio::main]
async fn main() -> Result<()> {
    ApplicationClient::start(ping_pong::application_descriptor()).await?;

    Ok(())
}
