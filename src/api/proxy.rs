use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use crate::config::AppState;
use crate::auth::AuthContext;
use crate::models::VirtualModel;
use unigateway_sdk::core::pool::ExecutionTarget;

pub async fn chat_completions(
    State(state): State<Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    let requested_model = payload.get("model").and_then(|m| m.as_str()).unwrap_or("");
    
    // 1. Resolve Virtual Model and authorize
    let virtual_model = match sqlx::query_as::<_, VirtualModel>(
        "SELECT vm.* FROM virtual_models vm 
         JOIN project_model_grants pmg ON vm.id = pmg.virtual_model_id
         WHERE vm.name = ? AND pmg.project_id = ? AND vm.enabled = 1"
    )
    .bind(requested_model)
    .bind(&auth.project.id)
    .fetch_optional(&state.db)
    .await {
        Ok(Some(vm)) => vm,
        Ok(None) => return (StatusCode::FORBIDDEN, "Access to this model is not granted or model not found").into_response(),
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response();
        }
    };

    // 2. Prepare UniGateway request
    let mut proxy_request = match unigateway_sdk::protocol::openai_payload_to_chat_request(&payload, requested_model) {
        Ok(req) => req,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Invalid request: {}", e)).into_response(),
    };

    // Attach enterprise metadata for hooks
    proxy_request.metadata.insert("org_id".to_string(), auth.project.org_id);
    proxy_request.metadata.insert("project_id".to_string(), auth.project.id);
    proxy_request.metadata.insert("key_id".to_string(), auth.api_key.id);
    proxy_request.metadata.insert("virtual_model_id".to_string(), virtual_model.id);
    proxy_request.metadata.insert("pool_id".to_string(), virtual_model.pool_id.clone());

    // 3. Dispatch to UniGateway
    let target = ExecutionTarget::Pool { pool_id: virtual_model.pool_id };
    
    match state.engine.proxy_chat(proxy_request, target).await {
        Ok(session) => {
            let response = unigateway_sdk::protocol::render_openai_chat_session(session);
            protocol_response_to_axum(response)
        }
        Err(e) => {
            tracing::error!("Proxy error: {}", e);
            (StatusCode::BAD_GATEWAY, format!("Upstream error: {}", e)).into_response()
        }
    }
}

fn protocol_response_to_axum(resp: unigateway_sdk::protocol::ProtocolHttpResponse) -> Response {
    use unigateway_sdk::protocol::ProtocolResponseBody;
    use axum::body::Body;
    
    let (status, body) = resp.into_parts();
    match body {
        ProtocolResponseBody::Json(json) => (status, Json(json)).into_response(),
        ProtocolResponseBody::ServerSentEvents(stream) => {
            let body = Body::from_stream(stream);
            Response::builder()
                .status(status)
                .header("content-type", "text/event-stream")
                .header("cache-control", "no-cache")
                .header("connection", "keep-alive")
                .body(body)
                .unwrap()
        }
    }
}
