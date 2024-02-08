mod handlers;
mod logger;
mod schemas;
mod settings;

use std::sync::Arc;

use axum::{
    middleware::{self},
    routing::post,
    Router,
};
use tokio::signal::{
    self,
    unix::{signal, SignalKind},
};

use crate::{
    handlers::{handle_configuration, handle_mock_request},
    schemas::new_shared_mock_server_state,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let settings = settings::Settings::new("mockser")?;

    log::info!("Starting with settings: {:?}", settings);

    let state = new_shared_mock_server_state();

    let app = Router::new()
        .layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            handle_mock_request,
        ))
        .with_state(Arc::clone(&state));

    let config_app = Router::new()
        .route("/configure", post(handle_configuration))
        .with_state(Arc::clone(&state));

    let addr = format!("{}:{}", settings.host, settings.port);
    log::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    let addr = format!("{}:{}", settings.host, settings.config_port);
    log::info!("Listening config on {}", addr);
    let config_listener = tokio::net::TcpListener::bind(addr).await?;

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::spawn(async move {
        axum::serve(config_listener, config_app).await.unwrap();
    });

    let mut shutdown_recv = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = signal::ctrl_c() => {},
        _ = shutdown_recv.recv() => {},
    }
    log::info!("Shutting down");

    Ok(())
}
