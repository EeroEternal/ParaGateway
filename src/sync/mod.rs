use std::sync::Arc;
use sqlx::SqlitePool;
use unigateway_sdk::core::{
    UniGatewayEngine, ProviderPool, Endpoint, ProviderKind, 
    LoadBalancingStrategy, RetryPolicy, ModelPolicy
};
use crate::models::{Endpoint as DbEndpoint, ModelPool as DbModelPool};

pub async fn sync_all_pools(engine: &UniGatewayEngine, db: &SqlitePool) -> anyhow::Result<()> {
    let pools = sqlx::query_as::<_, DbModelPool>("SELECT * FROM model_pools WHERE enabled = 1")
        .fetch_all(db)
        .await?;

    for pool in pools {
        let endpoints = sqlx::query_as::<_, DbEndpoint>(
            "SELECT e.*, mpe.priority, mpe.weight 
             FROM endpoints e 
             JOIN model_pool_endpoints mpe ON e.id = mpe.endpoint_id 
             WHERE mpe.pool_id = ?"
        )
        .bind(&pool.id)
        .fetch_all(db)
        .await?;

        let unigateway_endpoints = endpoints.iter().map(|e| {
            Endpoint {
                endpoint_id: e.id.clone(),
                provider_name: Some(e.name.clone()),
                source_endpoint_id: Some(e.upstream_model_id.clone()),
                provider_family: None,
                provider_kind: ProviderKind::OpenAiCompatible,
                driver_id: "openai".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
                api_key: unigateway_sdk::core::SecretString::new("REDACTED".to_string()),
                model_policy: ModelPolicy::default(),
                enabled: e.enabled,
                max_concurrency: None,
                metadata: std::collections::HashMap::new(),
            }
        }).collect();

        let strategy = match pool.strategy.as_str() {
            "score_ordered" => LoadBalancingStrategy::ScoreOrdered,
            "least_connections" => LoadBalancingStrategy::Fallback, // Feedback handled by provider
            _ => LoadBalancingStrategy::RoundRobin,
        };

        let provider_pool = ProviderPool {
            pool_id: pool.id.clone(),
            endpoints: unigateway_endpoints,
            load_balancing: strategy,
            retry_policy: RetryPolicy::default(),
            metadata: std::collections::HashMap::new(),
        };

        engine.upsert_pool(provider_pool).await?;
    }
    Ok(())
}
