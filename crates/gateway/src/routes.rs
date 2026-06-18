// HTTP route handlers for GitHub webhooks and gateway health checks.
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use metrics::counter;
use serde_json::json;
use tracing::{error, info, warn};
use waveflow_shared::{GitHubPullRequestEvent, WaveFlowError, WaveFlowResult};

use crate::attestation::{
    build_attestation, load_reward_per_point, persist_payout, persist_webhook_event, payout_exists,
    resolve_contributor_address, resolve_program_id, submit_attestation,
};
use crate::state::AppState;
use crate::webhook::{parse_merge_event, verify_github_signature};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/webhooks/github", post(github_webhook))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .is_ok();

    let status = if db_ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (
        status,
        Json(json!({
            "service": "waveflow-gateway",
            "database": db_ok,
        })),
    )
}

async fn github_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    counter!("waveflow_gateway_webhook_total").increment(1);

    let signature = headers
        .get("x-hub-signature-256")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if let Err(err) = verify_github_signature(&state.config.github_webhook_secret, &body, signature) {
        counter!("waveflow_gateway_webhook_rejected_total").increment(1);
        return map_error(err);
    }

    let event_type = headers
        .get("x-github-event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let delivery_id = headers
        .get("x-github-delivery")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);

    let payload: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(err) => {
            return map_error(WaveFlowError::Webhook(format!("invalid JSON: {err}")));
        }
    };

    if event_type != "pull_request" {
        let _ = persist_webhook_event(
            &state.db,
            delivery_id.as_deref(),
            &event_type,
            "unknown",
            None,
            payload,
            "ignored",
            Some("unsupported event type"),
        )
        .await;
        return (StatusCode::OK, Json(json!({ "status": "ignored" }))).into_response();
    }

    let pr_event: GitHubPullRequestEvent = match serde_json::from_value(payload.clone()) {
        Ok(v) => v,
        Err(err) => {
            return map_error(WaveFlowError::Webhook(format!("invalid pull_request payload: {err}")));
        }
    };

    match process_merge(&state, &pr_event, payload, delivery_id.as_deref()).await {
        Ok(response) => {
            counter!("waveflow_gateway_payout_success_total").increment(1);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            if matches!(err, WaveFlowError::Validation(_)) {
                counter!("waveflow_gateway_webhook_ignored_total").increment(1);
                (
                    StatusCode::OK,
                    Json(json!({ "status": "ignored", "reason": err.to_string() })),
                )
                    .into_response()
            } else {
                counter!("waveflow_gateway_webhook_failed_total").increment(1);
                error!(error = %err, "webhook processing failed");
                map_error(err)
            }
        }
    }
}

async fn process_merge(
    state: &AppState,
    event: &GitHubPullRequestEvent,
    raw_payload: serde_json::Value,
    delivery_id: Option<&str>,
) -> WaveFlowResult<serde_json::Value> {
    let merge = parse_merge_event(event)?;
    let (program_uuid, on_chain_program_id) =
        resolve_program_id(&state.db, &merge.github_repo).await?;

    if payout_exists(&state.db, program_uuid, merge.pr_number).await? {
        return Err(WaveFlowError::Conflict(format!(
            "PR {} already paid",
            merge.pr_number
        )));
    }

    let stellar_address =
        resolve_contributor_address(&state.db, program_uuid, &merge.github_username).await?;

    let attestation = build_attestation(on_chain_program_id, &merge);
    let tx_hash = submit_attestation(
        &state.config.soroban_rpc_url,
        state.config.escrow_contract_id.as_deref(),
        state.config.gateway_secret_key.as_deref(),
        &attestation,
    )
    .await?;

    let reward_per_point = load_reward_per_point(&state.db, program_uuid).await?;
    let amount = i128::from(merge.points) * reward_per_point;
    let payout_id = persist_payout(
        &state.db,
        program_uuid,
        &merge,
        &stellar_address,
        amount,
        &tx_hash,
    )
    .await?;

    persist_webhook_event(
        &state.db,
        delivery_id,
        "pull_request",
        &merge.github_repo,
        Some(merge.pr_number),
        raw_payload,
        "processed",
        None,
    )
    .await?;

    info!(
        payout_id = %payout_id,
        pr = merge.pr_number,
        repo = %merge.github_repo,
        "merge attestation processed"
    );

    Ok(json!({
        "status": "processed",
        "payout_id": payout_id,
        "tx_hash": tx_hash,
        "program_id": program_uuid,
    }))
}

fn map_error(err: WaveFlowError) -> axum::response::Response {
    let status = StatusCode::from_u16(err.http_status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    if status.is_server_error() {
        warn!(error = %err, "request failed");
    }
    (status, Json(json!({ "error": err.to_string() }))).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::webhook::verify_github_signature;

    #[test]
    fn error_maps_to_http_status() {
        let err = WaveFlowError::Unauthorized("nope".into());
        assert_eq!(err.http_status_code(), 401);
    }
}
