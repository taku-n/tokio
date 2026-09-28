use anyhow::*;

#[tokio::main]
async fn main() -> Result<()> {
    dbg!("hello, world");
    //Err(anyhow!("An error"))
    Ok(())
}
