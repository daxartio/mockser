mod handlers;
mod initial_configs;
mod logger;
mod schemas;
mod settings;

use std::sync::Arc;

use axum::{
    middleware::{self},
    routing::post,
    Router,
};
use tokio::signal;

use crate::{
    handlers::{handle_configuration, handle_mock_request},
    initial_configs::new_shared_mock_server_state_from_file,
    schemas::new_shared_mock_server_state,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let settings = settings::Settings::new("mockser")?;

    log::info!("Starting mockser {} with settings: {:?}", VERSION, settings);

    let state = if let Some(initial_configs) = settings.initial_configs {
        match new_shared_mock_server_state_from_file(initial_configs).await {
            Ok(state) => state,
            Err(e) => {
                log::error!("Failed to load initial configs: {}", e);
                new_shared_mock_server_state()
            }
        }
    } else {
        new_shared_mock_server_state()
    };

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

    match signal::ctrl_c().await {
        Ok(()) => {
            log::info!("Shutting down");
        }
        Err(err) => {
            log::error!("Unable to listen for shutdown signal: {}", err)
        }
    }

    Ok(())
}
