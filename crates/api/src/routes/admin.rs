// Admin-only routes for program registration (protected by API key middleware).
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use waveflow_shared::{validate_github_repo, WaveFlowError};

use crate::routes::ApiError;
use crate::state::AppState;

pub fn admin_router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/admin/programs", post(create_program))
        .route("/api/v1/admin/contributors", post(register_contributor))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct CreateProgramRequest {
    on_chain_program_id: u64,
    github_repo: String,
    maintainer_address: String,
    reward_per_point: i64,
    milestone_cap: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreateProgramResponse {
    id: Uuid,
}

async fn create_program(
    State(state): State<AppState>,
    Json(body): Json<CreateProgramRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if !validate_github_repo(&body.github_repo) {
        return Err(WaveFlowError::Validation("invalid github_repo slug".into()).into());
    }
    if body.reward_per_point <= 0 {
        return Err(WaveFlowError::Validation("reward_per_point must be positive".into()).into());
    }

    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO programs (id, on_chain_program_id, github_repo, maintainer_address, reward_per_point, milestone_cap)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(id)
    .bind(i64::try_from(body.on_chain_program_id).unwrap_or(i64::MAX))
    .bind(&body.github_repo)
    .bind(&body.maintainer_address)
    .bind(body.reward_per_point)
    .bind(body.milestone_cap)
    .execute(&state.db)
    .await
    .map_err(|e| WaveFlowError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(CreateProgramResponse { id })))
}

#[derive(Debug, Deserialize)]
struct RegisterContributorRequest {
    program_id: Uuid,
    github_username: String,
    stellar_address: String,
}

#[derive(Debug, Serialize)]
struct RegisterContributorResponse {
    program_id: Uuid,
    github_username: String,
}

async fn register_contributor(
    State(state): State<AppState>,
    Json(body): Json<RegisterContributorRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if body.github_username.is_empty() || body.stellar_address.is_empty() {
        return Err(WaveFlowError::Validation("username and address required".into()).into());
    }

    sqlx::query(
        r#"
        INSERT INTO contributors (program_id, github_username, stellar_address)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(body.program_id)
    .bind(&body.github_username)
    .bind(&body.stellar_address)
    .execute(&state.db)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.constraint().is_some() {
                return WaveFlowError::Conflict("contributor already registered".into());
            }
        }
        WaveFlowError::Database(e.to_string())
    })?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterContributorResponse {
            program_id: body.program_id,
            github_username: body.github_username,
        }),
    ))
}
