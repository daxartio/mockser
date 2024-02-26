use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use crate::schemas::{Mock, MockToDelete, SharedMockServerState};

pub async fn handle_configuration(
    State(state): State<SharedMockServerState>,
    Json(config): Json<Mock>,
) -> impl IntoResponse {
    let path = config.request.path.clone();
    let method = config.request.method.clone();

    log::info!("Configure updated for {}", path);

    state.write().await.configs.insert((path, method), config);

    StatusCode::CREATED
}

pub async fn handle_delete_configuration(
    State(state): State<SharedMockServerState>,
    Json(config): Json<MockToDelete>,
) -> impl IntoResponse {
    let path = config.request.path.clone();
    let method = config.request.method.clone();

    log::info!("Configure deleted for {}", path);

    state.write().await.configs.remove(&(path, method));

    StatusCode::NO_CONTENT
}

pub async fn handle_clear(State(state): State<SharedMockServerState>) -> impl IntoResponse {
    log::info!("Clearing all configurations");

    state.write().await.configs.clear();

    StatusCode::NO_CONTENT
}

pub async fn handle_mock_request(
    State(state): State<SharedMockServerState>,
    req: Request,
    _next: Next,
) -> impl IntoResponse {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    let state = &state.read().await;
    let configs = &state.configs;
    let config = match configs.get(&(path.clone(), method)) {
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

    if let Some(headers) = config.request.headers {
        if compare_headers(
            &headers,
            &req.headers()
                .iter()
                .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
                .collect(),
        ) {
            log::info!("Headers match for {}", path);
        } else {
            log::error!("Headers do not match for {}", path);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    }

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

fn compare_headers(expected: &HashMap<String, String>, actual: &HashMap<String, String>) -> bool {
    expected
        .iter()
        .all(|(key, value)| actual.get(key).map_or(false, |v| v == value))
}
