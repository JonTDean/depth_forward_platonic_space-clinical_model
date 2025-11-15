use crate::{client::BackendClient, config::AppConfig};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub client: BackendClient,
}

impl AppState {
    pub fn new(config: AppConfig, client: BackendClient) -> Self {
        Self { config, client }
    }
}
