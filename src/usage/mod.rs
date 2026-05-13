use crate::models::EndpointMetric;
use dashmap::DashMap;
use futures_util::future::BoxFuture;
use sqlx::SqlitePool;
use std::sync::Arc;
use unigateway_sdk::core::hooks::{
    AttemptFinishedEvent, AttemptStartedEvent, GatewayHooks,
};
use unigateway_sdk::core::response::RequestReport;

pub struct ParaGatewayHooks {
    pub db: SqlitePool,
    pub metrics: Arc<DashMap<String, EndpointMetric>>,
}

impl GatewayHooks for ParaGatewayHooks {
    fn on_attempt_started(&self, event: AttemptStartedEvent) -> BoxFuture<'static, ()> {
        let metrics = self.metrics.clone();
        let endpoint_id = event.endpoint_id.clone();
        let active = event.active_attempts_at_start;

        Box::pin(async move {
            let mut metric = metrics.entry(endpoint_id.clone()).or_insert(EndpointMetric {
                endpoint_id,
                active_requests: 0,
                ema_latency_ms: 0.0,
                total_requests: 0,
                total_errors: 0,
                last_error_at: None,
                updated_at: chrono::Utc::now(),
            });
            // UniGateway 2.1.0 provides an accurate snapshot of active attempts
            metric.active_requests = active as i32;
            metric.total_requests += 1;
        })
    }

    fn on_attempt_finished(&self, event: AttemptFinishedEvent) -> BoxFuture<'static, ()> {
        let metrics = self.metrics.clone();
        let endpoint_id = event.endpoint_id.clone();
        let latency = event.latency_ms as f64;
        let success = event.success;

        Box::pin(async move {
            if let Some(mut metric) = metrics.get_mut(&endpoint_id) {
                if metric.active_requests > 0 {
                    metric.active_requests -= 1;
                }

                if metric.ema_latency_ms == 0.0 {
                    metric.ema_latency_ms = latency;
                } else {
                    let alpha = 0.1;
                    metric.ema_latency_ms = (1.0 - alpha) * metric.ema_latency_ms + alpha * latency;
                }

                if !success {
                    metric.total_errors += 1;
                    metric.last_error_at = Some(chrono::Utc::now());
                }
                
                metric.updated_at = chrono::Utc::now();
            }
        })
    }

    fn on_request_finished(&self, report: RequestReport) -> BoxFuture<'static, ()> {
        let db = self.db.clone();
        
        Box::pin(async move {
            let metadata = serde_json::to_string(&report.metadata).unwrap_or_default();
            
            let res = sqlx::query(
                "INSERT INTO usage_logs (
                    id, org_id, project_id, key_id, virtual_model_id, 
                    pool_id, endpoint_id, provider_account_id, 
                    prompt_tokens, completion_tokens, total_tokens, 
                    latency_ms, status_code, error_message, metadata
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(report.metadata.get("org_id"))
            .bind(report.metadata.get("project_id"))
            .bind(report.metadata.get("key_id"))
            .bind(report.metadata.get("virtual_model_id"))
            .bind(report.metadata.get("pool_id"))
            .bind(report.metadata.get("endpoint_id"))
            .bind(report.metadata.get("provider_account_id"))
            .bind(report.usage.as_ref().and_then(|u| u.input_tokens).unwrap_or(0) as i32)
            .bind(report.usage.as_ref().and_then(|u| u.output_tokens).unwrap_or(0) as i32)
            .bind(report.usage.as_ref().and_then(|u| u.total_tokens).unwrap_or(0) as i32)
            .bind(report.latency_ms as i32)
            .bind(200)
            .bind(None::<String>)
            .bind(metadata)
            .execute(&db)
            .await;

            if let Err(e) = res {
                tracing::error!("Failed to log usage: {}", e);
            }
        })
    }
}
