use anyhow::Result;
use blt::{application_factory, application_factory::ApplicationMode};

#[tokio::main]
async fn main() -> Result<()> {
    application_factory::launch_application(ApplicationMode::Server).await?;
    Ok(())
}
