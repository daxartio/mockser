#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

#[cfg(unix)]
pub async fn wait() -> Result<(), Box<dyn std::error::Error>> {
    let mut shutdown_recv = signal(SignalKind::terminate())?;

    tokio::select! {
        res = tokio::signal::ctrl_c() => {
            if let Err(err) = res {
                return Err(err.into());
            }
        },
        _ = shutdown_recv.recv() => {},
    }
    Ok(())
}

#[cfg(windows)]
pub async fn wait() -> Result<(), Box<dyn std::error::Error>> {
    tokio::signal::ctrl_c().await?;
    Ok(())
}
