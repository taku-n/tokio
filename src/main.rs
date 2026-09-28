use anyhow::*;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
            .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
                    time::UtcOffset::from_hms(9, 0, 0).unwrap(),
                    time::format_description::well_known::Rfc3339,
            )).init();

    info!("hello, world");

    //Err(anyhow!("An error"))
    Ok(())
}
