// Application configuration loaded from environment variables.
use std::env;

use crate::error::{WaveFlowError, WaveFlowResult};

pub const TESTNET_NETWORK_PASSPHRASE: &str = "Test SDF Network ; September 2015";
pub const PUBLIC_NETWORK_PASSPHRASE: &str = "Public Global Stellar Network ; September 2015";

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
        let github_webhook_secret = env::var("GITHUB_WEBHOOK_SECRET")
            .map_err(|_| WaveFlowError::Config("GITHUB_WEBHOOK_SECRET is required".into()))?;
        let soroban_rpc_url = env::var("SOROBAN_RPC_URL")
            .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".into());
        let network_passphrase = env::var("NETWORK_PASSPHRASE")
            .unwrap_or_else(|_| TESTNET_NETWORK_PASSPHRASE.into());
        validate_rpc_network_pair(&soroban_rpc_url, &network_passphrase)?;
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

fn validate_rpc_network_pair(soroban_rpc_url: &str, network_passphrase: &str) -> WaveFlowResult<()> {
    let lower_url = soroban_rpc_url.to_ascii_lowercase();

    if lower_url.contains("mainnet") && network_passphrase != PUBLIC_NETWORK_PASSPHRASE {
        return Err(WaveFlowError::Config(format!(
            "NETWORK_PASSPHRASE must be `{PUBLIC_NETWORK_PASSPHRASE}` when SOROBAN_RPC_URL points to mainnet"
        )));
    }

    if lower_url.contains("testnet") && network_passphrase != TESTNET_NETWORK_PASSPHRASE {
        return Err(WaveFlowError::Config(format!(
            "NETWORK_PASSPHRASE must be `{TESTNET_NETWORK_PASSPHRASE}` when SOROBAN_RPC_URL points to testnet"
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
        std::env::set_var("SOROBAN_RPC_URL", "https://soroban-testnet.stellar.org");
        std::env::set_var("NETWORK_PASSPHRASE", TESTNET_NETWORK_PASSPHRASE);
        std::env::set_var("API_ADMIN_KEYS", "key-a, key-b");

        let cfg = AppConfig::from_env().expect("config");
        assert_eq!(cfg.api_admin_keys, vec!["key-a", "key-b"]);

        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("GITHUB_WEBHOOK_SECRET");
        std::env::remove_var("SOROBAN_RPC_URL");
        std::env::remove_var("NETWORK_PASSPHRASE");
        std::env::remove_var("API_ADMIN_KEYS");
    }

    #[test]
    fn rejects_mainnet_rpc_with_testnet_passphrase() {
        let err = validate_rpc_network_pair(
            "https://soroban-mainnet.stellar.org",
            TESTNET_NETWORK_PASSPHRASE,
        )
        .expect_err("mainnet RPC requires public passphrase");

        assert!(err.to_string().contains(PUBLIC_NETWORK_PASSPHRASE));
    }

    #[test]
    fn rejects_testnet_rpc_with_public_passphrase() {
        let err = validate_rpc_network_pair(
            "https://soroban-testnet.stellar.org",
            PUBLIC_NETWORK_PASSPHRASE,
        )
        .expect_err("testnet RPC requires testnet passphrase");

        assert!(err.to_string().contains(TESTNET_NETWORK_PASSPHRASE));
    }

    #[test]
    fn accepts_matching_mainnet_rpc_and_passphrase() {
        validate_rpc_network_pair(
            "https://soroban-mainnet.stellar.org",
            PUBLIC_NETWORK_PASSPHRASE,
        )
        .expect("mainnet RPC with public passphrase");
    }
}
