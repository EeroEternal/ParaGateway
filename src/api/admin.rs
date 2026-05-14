use axum::{
    extract::{State, Json},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::config::AppState;
use crate::models::{ProviderAccount, ModelPool, Endpoint, VirtualModel, Org, Project, ApiKey};
use crate::api::models::{
    ApiResponse, CreateProviderReq, CreatePoolReq, CreateEndpointReq, CreateVirtualModelReq,
    CreateOrgReq, CreateProjectReq, CreateApiKeyReq, ApiKeyResponse,
    BindEndpointToPoolReq, GrantModelToProjectReq
};

pub fn admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Resources
        .route("/providers", get(list_providers).post(create_provider))
        .route("/endpoints", get(list_endpoints).post(create_endpoint))
        // Orchestration
        .route("/pools", get(list_pools).post(create_pool))
        .route("/pools/bind", post(bind_endpoint_to_pool))
        .route("/virtual-models", get(list_virtual_models).post(create_virtual_model))
        // Access
        .route("/orgs", get(list_orgs).post(create_org))
        .route("/projects", get(list_projects).post(create_project))
        .route("/projects/grant", post(grant_model_to_project))
        .route("/api-keys", get(list_api_keys).post(create_api_key))
}

// Providers
async fn list_providers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<ProviderAccount>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let providers = sqlx::query_as::<_, ProviderAccount>("SELECT * FROM provider_accounts")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
        })?;

    Ok(Json(ApiResponse::success(providers)))
}

async fn create_provider(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProviderReq>,
) -> Result<Json<ApiResponse<ProviderAccount>>, (StatusCode, Json<ApiResponse<()>>)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO provider_accounts (id, name, provider_type, base_url, api_key) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.provider_type)
    .bind(&payload.base_url)
    .bind(&payload.api_key)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
    })?;

    let provider = sqlx::query_as::<_, ProviderAccount>("SELECT * FROM provider_accounts WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    let provider = sqlx::query_as::<_, ProviderAccount>("SELECT * FROM provider_accounts WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    // Sync to engine
    let _ = crate::sync::sync_all_pools(&state.engine, &state.db).await;

    Ok(Json(ApiResponse::success(provider)))
}

// Pools
async fn list_pools(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<ModelPool>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let pools = sqlx::query_as::<_, ModelPool>("SELECT * FROM model_pools")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
        })?;

    Ok(Json(ApiResponse::success(pools)))
}

async fn create_pool(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePoolReq>,
) -> Result<Json<ApiResponse<ModelPool>>, (StatusCode, Json<ApiResponse<()>>)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO model_pools (id, name, strategy) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.strategy)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
    })?;

    let pool = sqlx::query_as::<_, ModelPool>("SELECT * FROM model_pools WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    // Sync to engine
    let _ = crate::sync::sync_all_pools(&state.engine, &state.db).await;

    Ok(Json(ApiResponse::success(pool)))
}

// Endpoints
async fn list_endpoints(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Endpoint>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let endpoints = sqlx::query_as::<_, Endpoint>("SELECT * FROM endpoints")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
        })?;

    Ok(Json(ApiResponse::success(endpoints)))
}

async fn create_endpoint(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateEndpointReq>,
) -> Result<Json<ApiResponse<Endpoint>>, (StatusCode, Json<ApiResponse<()>>)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO endpoints (id, account_id, name, upstream_model_id, priority, weight) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.account_id)
    .bind(&payload.name)
    .bind(&payload.upstream_model_id)
    .bind(payload.priority.unwrap_or(1))
    .bind(payload.weight.unwrap_or(1))
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
    })?;

    let endpoint = sqlx::query_as::<_, Endpoint>("SELECT * FROM endpoints WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    // Sync to engine
    let _ = crate::sync::sync_all_pools(&state.engine, &state.db).await;

    Ok(Json(ApiResponse::success(endpoint)))
}

// Virtual Models
async fn list_virtual_models(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<VirtualModel>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let models = sqlx::query_as::<_, VirtualModel>("SELECT * FROM virtual_models")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
        })?;

    Ok(Json(ApiResponse::success(models)))
}

async fn create_virtual_model(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateVirtualModelReq>,
) -> Result<Json<ApiResponse<VirtualModel>>, (StatusCode, Json<ApiResponse<()>>)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO virtual_models (id, pool_id, name) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.pool_id)
    .bind(&payload.name)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
    })?;

    let model = sqlx::query_as::<_, VirtualModel>("SELECT * FROM virtual_models WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    // Sync to engine
    let _ = crate::sync::sync_all_pools(&state.engine, &state.db).await;

    Ok(Json(ApiResponse::success(model)))
}

// Access Control & Mappings
async fn bind_endpoint_to_pool(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BindEndpointToPoolReq>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    sqlx::query(
        "INSERT INTO model_pool_endpoints (pool_id, endpoint_id, priority, weight) VALUES (?, ?, ?, ?)
         ON CONFLICT(pool_id, endpoint_id) DO UPDATE SET priority=excluded.priority, weight=excluded.weight"
    )
    .bind(&payload.pool_id)
    .bind(&payload.endpoint_id)
    .bind(payload.priority.unwrap_or(1))
    .bind(payload.weight.unwrap_or(1))
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
    })?;

    // Sync to engine
    let _ = crate::sync::sync_all_pools(&state.engine, &state.db).await;

    Ok(Json(ApiResponse::success(())))
}


async fn list_orgs(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Org>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let orgs = sqlx::query_as::<_, Org>("SELECT * FROM orgs")
        .fetch_all(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    Ok(Json(ApiResponse::success(orgs)))
}

async fn create_org(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrgReq>,
) -> Result<Json<ApiResponse<Org>>, (StatusCode, Json<ApiResponse<()>>)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query("INSERT INTO orgs (id, name, description) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&payload.name)
        .bind(&payload.description)
        .execute(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    let org = sqlx::query_as::<_, Org>("SELECT * FROM orgs WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    Ok(Json(ApiResponse::success(org)))
}

async fn list_projects(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Project>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    Ok(Json(ApiResponse::success(projects)))
}

async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProjectReq>,
) -> Result<Json<ApiResponse<Project>>, (StatusCode, Json<ApiResponse<()>>)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query("INSERT INTO projects (id, org_id, name, description) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&payload.org_id)
        .bind(&payload.name)
        .bind(&payload.description)
        .execute(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    Ok(Json(ApiResponse::success(project)))
}

async fn grant_model_to_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GrantModelToProjectReq>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    sqlx::query("INSERT OR IGNORE INTO project_model_grants (project_id, virtual_model_id) VALUES (?, ?)")
        .bind(&payload.project_id)
        .bind(&payload.virtual_model_id)
        .execute(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    Ok(Json(ApiResponse::success(())))
}

async fn list_api_keys(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<ApiKey>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let keys = sqlx::query_as::<_, ApiKey>("SELECT * FROM api_keys")
        .fetch_all(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error"))))?;

    Ok(Json(ApiResponse::success(keys)))
}

use crate::auth::hash_token;

async fn create_api_key(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateApiKeyReq>,
) -> Result<Json<ApiResponse<ApiKeyResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    // Generate a secure random key
    let raw_key = format!("pk_{}", uuid::Uuid::new_v4().simple());
    let key_hash = hash_token(&raw_key);
    let key_prefix = raw_key[..7].to_string(); // "pk_xxxx"
    
    sqlx::query(
        "INSERT INTO api_keys (id, project_id, name, key_hash, key_prefix) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.project_id)
    .bind(&payload.name)
    .bind(&key_hash)
    .bind(&key_prefix)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Database error")))
    })?;

    let response = ApiKeyResponse {
        id,
        project_id: payload.project_id,
        name: payload.name,
        key: raw_key, // Only returned once
        key_prefix,
    };

    Ok(Json(ApiResponse::success(response)))
}
