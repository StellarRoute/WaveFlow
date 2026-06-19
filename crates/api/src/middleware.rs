// API key authentication middleware for admin routes.
use axum::{
    body::Body,
    extract::State,
    http::{HeaderName, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::state::AppState;

pub async fn require_admin_key(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    if state.config.api_admin_keys.is_empty() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "API_ADMIN_KEYS not configured" })),
        )
            .into_response();
    }

    let authorized = req
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|key| state.config.api_admin_keys.iter().any(|k| k == key));

    if !authorized {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "invalid or missing x-api-key" })),
        )
            .into_response();
    }

    next.run(req).await
}

pub async fn security_headers(req: Request<Body>, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("no-referrer"),
    );
    headers.insert(
        HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static("frame-ancestors 'none'"),
    );

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use waveflow_shared::AppConfig;

    #[tokio::test]
    async fn rejects_missing_api_key() {
        let config = AppConfig {
            database_url: "postgres://localhost/waveflow".into(),
            github_webhook_secret: "secret".into(),
            soroban_rpc_url: "http://localhost".into(),
            network_passphrase: "Test".into(),
            escrow_contract_id: None,
            gateway_secret_key: None,
            api_admin_keys: vec!["admin-key".into()],
            gateway_port: 8080,
            api_port: 8081,
        };

        let app = Router::new().route("/admin", get(|| async { "ok" })).layer(
            axum::middleware::from_fn_with_state(
                AppState::new(
                    config,
                    sqlx::PgPool::connect_lazy("postgres://localhost/waveflow").unwrap(),
                ),
                require_admin_key,
            ),
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/admin")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn applies_security_headers_without_changing_json_body() {
        let app = Router::new()
            .route("/json", get(|| async { Json(json!({ "status": "ok" })) }))
            .layer(axum::middleware::from_fn(security_headers));

        let response = app
            .oneshot(Request::builder().uri("/json").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("x-content-type-options").unwrap(),
            "nosniff"
        );
        assert_eq!(response.headers().get("x-frame-options").unwrap(), "DENY");
        assert_eq!(
            response.headers().get("referrer-policy").unwrap(),
            "no-referrer"
        );
        assert_eq!(
            response.headers().get("content-security-policy").unwrap(),
            "frame-ancestors 'none'"
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], br#"{"status":"ok"}"#);
    }
}
