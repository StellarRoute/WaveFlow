// Gateway startup validation before accepting webhook traffic.
use tracing::warn;
use waveflow_shared::{AppConfig, WaveFlowResult};

pub fn validate_gateway_config(config: &AppConfig) -> WaveFlowResult<()> {
    if config.github_webhook_secret.is_empty() {
        warn!("GITHUB_WEBHOOK_SECRET is empty; webhook signature verification will reject all deliveries");
    }
    if config.escrow_contract_id.is_some() && config.gateway_secret_key.is_none() {
        warn!("ESCROW_CONTRACT_ID is set but GATEWAY_SECRET_KEY is missing; chain submission stays in dry-run mode");
    }
    Ok(())
}
