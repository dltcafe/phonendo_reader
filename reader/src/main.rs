use anyhow::Result;
use blt::application_factory::ApplicationFactory;

#[tokio::main]
async fn main() -> Result<()> {
    ApplicationFactory::launch_application().await?;
    Ok(())
}
