mod logger;
mod settings;

use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use tokio::{
    signal::{
        self,
        unix::{signal, SignalKind},
    },
    sync::RwLock,
};

#[derive(serde::Deserialize, Clone)]
struct Mock {
    #[serde(default)]
    name: String,
    request: MockRequest,
    response: MockResponse,
}

#[derive(serde::Deserialize, Clone)]
#[allow(unused)]
struct MockRequest {
    path: String,
    #[serde(default = "default_method")]
    method: String,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Clone)]
struct MockResponse {
    code: u16,
    body: String,
    headers: HashMap<String, String>,
}

type SharedMockServerState = Arc<RwLock<MockServerState>>;

#[derive(Clone)]
struct MockServerState {
    configs: HashMap<String, Mock>,
}

impl MockServerState {
    fn new() -> Self {
        MockServerState {
            configs: HashMap::new(),
        }
    }
}

fn default_method() -> String {
    "GET".to_string()
}

async fn configure_mock(
    State(state): State<SharedMockServerState>,
    Json(config): Json<Mock>,
) -> impl IntoResponse {
    let path = config.request.path.clone();

    log::info!("Configure updated for {}", path);

    state.write().await.configs.insert(path, config);

    StatusCode::CREATED
}

async fn handle(
    State(state): State<SharedMockServerState>,
    req: Request,
    _next: Next,
) -> impl IntoResponse {
    let path = req.uri().path().to_string();

    let state = &state.read().await;
    let configs = &state.configs;
    let config = match configs.get(&path) {
        Some(config) => config.clone(),
        None => {
            log::error!("No config found for {}", path);
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
        }
    };

    log::info!("Config found for {} - {}", path, config.name);

    let response = axum::http::Response::builder().status(config.response.code);

    let response = config
        .response
        .headers
        .into_iter()
        .fold(response, |response, (key, value)| {
            response.header(key, HeaderValue::from_str(&value).unwrap())
        });

    response.body(Body::from(config.response.body)).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let settings = settings::Settings::new("mockser")?;

    log::info!("Starting with settings: {:?}", settings);

    let state = SharedMockServerState::new(RwLock::new(MockServerState::new()));

    let app = Router::new()
        .layer(middleware::from_fn_with_state(Arc::clone(&state), handle))
        .with_state(Arc::clone(&state));

    let config_app = Router::new()
        .route("/configure", post(configure_mock))
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
