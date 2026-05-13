use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Org {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Project {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ApiKey {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub key_hash: String,
    pub key_prefix: String,
    pub enabled: bool,
    pub metadata: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ProviderAccount {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub base_url: String,
    pub api_key: String,
    pub status: String,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Endpoint {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub upstream_model_id: String,
    pub enabled: bool,
    pub priority: i32,
    pub weight: i32,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ModelPool {
    pub id: String,
    pub name: String,
    pub strategy: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct VirtualModel {
    pub id: String,
    pub pool_id: String,
    pub name: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UsageLog {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub org_id: Option<String>,
    pub project_id: Option<String>,
    pub key_id: Option<String>,
    pub virtual_model_id: Option<String>,
    pub pool_id: Option<String>,
    pub endpoint_id: Option<String>,
    pub provider_account_id: Option<String>,
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub latency_ms: i32,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct EndpointMetric {
    pub endpoint_id: String,
    pub active_requests: i32,
    pub ema_latency_ms: f64,
    pub total_requests: i32,
    pub total_errors: i32,
    pub last_error_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}
