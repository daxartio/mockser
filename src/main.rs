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
    sync::Mutex,
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

#[derive(Clone)]
struct MockServerState {
    configs: Arc<Mutex<HashMap<String, Mock>>>,
}

impl MockServerState {
    fn new() -> Self {
        MockServerState {
            configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

fn default_method() -> String {
    "GET".to_string()
}

async fn configure_mock(
    State(state): State<MockServerState>,
    Json(config): Json<Mock>,
) -> impl IntoResponse {
    let mut configs = state.configs.lock().await;
    let path = config.request.path.clone();

    log::info!("Configure updated for {}", path);

    configs.insert(path, config);

    StatusCode::CREATED
}

async fn handle(
    State(state): State<MockServerState>,
    req: Request,
    _next: Next,
) -> impl IntoResponse {
    let path = req.uri().path().to_string();

    let configs = state.configs.lock().await;
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
    let settings = settings::Settings::new("mockser").unwrap();

    let state = MockServerState::new();

    let app = Router::new()
        .layer(middleware::from_fn_with_state(state.clone(), handle))
        .with_state(state.clone());

    let config_app = Router::new()
        .route("/configure", post(configure_mock))
        .with_state(state.clone());

    let addr = format!("{}:{}", settings.host, settings.port);
    log::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let addr = format!("{}:{}", settings.host, settings.config_port);
    log::info!("Listening config on {}", addr);
    let config_listener = tokio::net::TcpListener::bind(addr).await.unwrap();

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
