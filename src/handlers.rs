use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use crate::schemas::{Mock, SharedMockServerState};

pub async fn handle_configuration(
    State(state): State<SharedMockServerState>,
    Json(config): Json<Mock>,
) -> impl IntoResponse {
    let path = config.request.path.clone();

    log::info!("Configure updated for {}", path);

    state.write().await.configs.insert(path, config);

    StatusCode::CREATED
}

pub async fn handle_mock_request(
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
