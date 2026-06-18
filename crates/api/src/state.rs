// API application state with database pool and configuration.
use sqlx::PgPool;
use std::sync::Arc;
use waveflow_shared::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: PgPool,
}

impl AppState {
    pub fn new(config: AppConfig, db: PgPool) -> Self {
        Self {
            config: Arc::new(config),
            db,
        }
    }
}
