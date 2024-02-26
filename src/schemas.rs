use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

#[derive(serde::Deserialize, Clone)]
pub struct Mock {
    #[serde(default)]
    pub name: String,
    pub request: MockRequest,
    pub response: MockResponse,
}

#[derive(serde::Deserialize, Clone)]
pub struct MockToDelete {
    pub request: MockRequest,
}

#[derive(serde::Deserialize, Clone)]
#[allow(unused)]
pub struct MockRequest {
    pub path: String,
    #[serde(default = "default_method")]
    pub method: String,
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Clone)]
pub struct MockResponse {
    pub code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

pub type SharedMockServerState = Arc<RwLock<MockServerState>>;

pub fn new_shared_mock_server_state() -> SharedMockServerState {
    Arc::new(RwLock::new(MockServerState::new()))
}

#[derive(Clone)]
pub struct MockServerState {
    pub configs: HashMap<String, Mock>,
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
