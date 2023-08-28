use anyhow::{anyhow, Context, Result};

use futures::{FutureExt, TryFutureExt};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {

    let mut sigterm_hook =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .context("failed to install SIGTERM handler")?;

    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
    let sender = shutdown_tx.clone();
    let sigterm = sigterm_hook
        .recv()
        .then(|_| async {
            info!("received SIGTERM, beginning graceful shutdown");
            sender.send(())
        })
        .map_err(|e| anyhow!(e));

    let config = tablet::Config::default();
    let tablet_served = tablet::server::serve(shutdown_tx, config);

    let _ = tokio::try_join!(sigterm, tablet_served)?;

    Ok(())
}
