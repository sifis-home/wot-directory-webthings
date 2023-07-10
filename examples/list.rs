use std::future::ready;

use futures_util::StreamExt;
use tracing::info;
use wot_discovery::discovery::Discovery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let d = Discovery::new()?;

    d.stream()?
        .for_each(|thing| {
            info!("* {:?}", thing);
            ready(())
        })
        .await;

    Ok(())
}
