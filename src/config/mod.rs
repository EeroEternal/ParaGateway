use serde::Deserialize;
use std::net::SocketAddr;
use sqlx::SqlitePool;
use dashmap::DashMap;
use std::sync::Arc;
use crate::models::{EndpointMetric, ModelPool};
use unigateway_sdk::core::UniGatewayEngine;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub addr: SocketAddr,
    pub database_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: SqlitePool,
    pub metrics: Arc<DashMap<String, EndpointMetric>>,
    pub pools: Arc<DashMap<String, ModelPool>>,
    pub engine: Arc<UniGatewayEngine>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        
        let addr = std::env::var("ADDR")
            .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
            .parse()?;
            
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:paragateway.db".to_string());

        Ok(Self {
            addr,
            database_url,
        })
    }
}
