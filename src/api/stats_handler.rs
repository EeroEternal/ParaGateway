use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use std::sync::Arc;
use crate::config::AppState;
use crate::api::models::ApiResponse;

pub async fn get_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ApiResponse<()>>)> {
    let row: (Option<i32>, Option<f64>, i32) = sqlx::query_as(
        "SELECT SUM(total_tokens), AVG(latency_ms), COUNT(*) FROM usage_logs"
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "total_tokens": row.0.unwrap_or(0),
        "avg_latency": row.1.unwrap_or(0.0),
        "request_count": row.2
    }))))
}
