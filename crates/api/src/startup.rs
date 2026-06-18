// Startup dependency checks before serving API traffic.
use sqlx::PgPool;
use waveflow_shared::{AppConfig, WaveFlowError, WaveFlowResult};

pub async fn run_startup_checks(config: &AppConfig, db: &PgPool) -> WaveFlowResult<()> {
    if config.database_url.is_empty() {
        return Err(WaveFlowError::Config("DATABASE_URL missing".into()));
    }
    sqlx::query("SELECT 1")
        .execute(db)
        .await
        .map_err(|e| WaveFlowError::Database(format!("database unreachable: {e}")))?;
    Ok(())
}
