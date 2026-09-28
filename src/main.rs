use anyhow::*;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("hello, world");

    //Err(anyhow!("An error"))
    Ok(())
}
