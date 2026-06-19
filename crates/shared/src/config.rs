// Application configuration loaded from environment variables.
use std::env;

use crate::error::{WaveFlowError, WaveFlowResult};

const DEFAULT_GITHUB_WEBHOOK_SECRET: &str = "change-me-to-github-webhook-secret";
const MIN_PRODUCTION_WEBHOOK_SECRET_LEN: usize = 32;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub github_webhook_secret: String,
    pub soroban_rpc_url: String,
    pub network_passphrase: String,
    pub escrow_contract_id: Option<String>,
    pub gateway_secret_key: Option<String>,
    pub api_admin_keys: Vec<String>,
    pub gateway_port: u16,
    pub api_port: u16,
}

impl AppConfig {
    pub fn from_env() -> WaveFlowResult<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| WaveFlowError::Config("DATABASE_URL is required".into()))?;
        let waveflow_env = env::var("WAVEFLOW_ENV").unwrap_or_else(|_| "development".into());
        let github_webhook_secret = env::var("GITHUB_WEBHOOK_SECRET")
            .map_err(|_| WaveFlowError::Config("GITHUB_WEBHOOK_SECRET is required".into()))?;
        validate_webhook_secret(&waveflow_env, &github_webhook_secret)?;
        let soroban_rpc_url = env::var("SOROBAN_RPC_URL")
            .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".into());
        let network_passphrase = env::var("NETWORK_PASSPHRASE")
            .unwrap_or_else(|_| "Test SDF Network ; September 2015".into());
        let escrow_contract_id = env::var("ESCROW_CONTRACT_ID").ok();
        let gateway_secret_key = env::var("GATEWAY_SECRET_KEY").ok();
        let api_admin_keys = env::var("API_ADMIN_KEYS")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        let gateway_port = env::var("GATEWAY_PORT")
            .or_else(|_| env::var("PORT"))
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);
        let api_port = env::var("API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8081);

        Ok(Self {
            database_url,
            github_webhook_secret,
            soroban_rpc_url,
            network_passphrase,
            escrow_contract_id,
            gateway_secret_key,
            api_admin_keys,
            gateway_port,
            api_port,
        })
    }
}

fn validate_webhook_secret(waveflow_env: &str, secret: &str) -> WaveFlowResult<()> {
    if !waveflow_env.eq_ignore_ascii_case("production") {
        return Ok(());
    }

    let trimmed_secret = secret.trim();

    if trimmed_secret == DEFAULT_GITHUB_WEBHOOK_SECRET || trimmed_secret.is_empty() {
        return Err(WaveFlowError::Config(
            "GITHUB_WEBHOOK_SECRET must be replaced with a random production secret".into(),
        ));
    }

    if trimmed_secret.len() < MIN_PRODUCTION_WEBHOOK_SECRET_LEN {
        return Err(WaveFlowError::Config(format!(
            "GITHUB_WEBHOOK_SECRET must be at least {MIN_PRODUCTION_WEBHOOK_SECRET_LEN} characters in production"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_admin_keys_from_comma_list() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/waveflow");
        std::env::set_var("GITHUB_WEBHOOK_SECRET", "secret");
        std::env::set_var("WAVEFLOW_ENV", "development");
        std::env::set_var("API_ADMIN_KEYS", "key-a, key-b");

        let cfg = AppConfig::from_env().expect("config");
        assert_eq!(cfg.api_admin_keys, vec!["key-a", "key-b"]);

        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("GITHUB_WEBHOOK_SECRET");
        std::env::remove_var("WAVEFLOW_ENV");
        std::env::remove_var("API_ADMIN_KEYS");
    }

    #[test]
    fn development_allows_example_webhook_secret() {
        validate_webhook_secret("development", DEFAULT_GITHUB_WEBHOOK_SECRET)
            .expect("example secret is allowed outside production");
    }

    #[test]
    fn production_rejects_example_webhook_secret() {
        let err = validate_webhook_secret("production", DEFAULT_GITHUB_WEBHOOK_SECRET)
            .expect_err("production must reject the example secret");

        assert!(err.to_string().contains("random production secret"));
    }

    #[test]
    fn production_rejects_short_webhook_secret() {
        let err = validate_webhook_secret("production", "short")
            .expect_err("production must reject short secrets");

        assert!(err.to_string().contains("at least 32 characters"));
    }

    #[test]
    fn production_accepts_strong_webhook_secret() {
        validate_webhook_secret(
            "production",
            "a-random-production-webhook-secret-with-entropy",
        )
        .expect("strong production secret");
    }
}
