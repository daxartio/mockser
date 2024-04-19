use crate::schemas::{new_shared_mock_server_state, Mock, SharedMockServerState};
use std::{collections::HashMap, path::PathBuf};

pub async fn new_shared_mock_server_state_from_file(
    file: PathBuf,
) -> Result<SharedMockServerState, Box<dyn std::error::Error>> {
    let state = new_shared_mock_server_state();
    let content = std::fs::read_to_string(file).unwrap();
    let mocks: Vec<Mock> = serde_json::from_str(&content)?;
    {
        let mut state = state.write().await;
        state.configs = HashMap::with_capacity(mocks.len());
        for mock in mocks {
            let path = mock.request.path.clone();
            let method = mock.request.method.clone();
            let query = mock.request.query.clone();
            state.configs.insert((path, method, query), mock);
        }
    }

    Ok(state)
}
