use crate::models::{EndpointMetric, ModelPool};
use dashmap::DashMap;
use std::sync::Arc;
use unigateway_sdk::core::feedback::{EndpointSignal, RoutingFeedback, RoutingFeedbackProvider};

pub struct ParaGatewayFeedbackProvider {
    pub metrics: Arc<DashMap<String, EndpointMetric>>,
    pub pools: Arc<DashMap<String, ModelPool>>,
}

impl RoutingFeedbackProvider for ParaGatewayFeedbackProvider {
    fn feedback(&self, pool_id: &str) -> RoutingFeedback {
        let mut feedback = RoutingFeedback::default();

        let strategy = self.pools.get(pool_id).map(|p| p.strategy.clone()).unwrap_or_else(|| "round_robin".to_string());

        for entry in self.metrics.iter() {
            let endpoint_id = entry.key();
            let metric = entry.value();

            let score = match strategy.as_str() {
                "least_connections" => {
                    1.0 / (1.0 + metric.active_requests as f64)
                }
                "latency_based" => {
                    if metric.ema_latency_ms > 0.0 {
                        1000.0 / metric.ema_latency_ms
                    } else {
                        1.0
                    }
                }
                _ => 0.0,
            };

            feedback.endpoint_signals.insert(
                endpoint_id.clone(),
                EndpointSignal {
                    score: Some(score),
                    excluded: false,
                    cooldown_until: None,
                    recent_error_rate: None,
                },
            );
        }

        feedback
    }
}
