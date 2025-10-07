use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub nats: NatsConfig,
    pub jwt: JwtConfig,
    pub blockchain: BlockchainConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsConfig {
    pub url: String,
    pub max_reconnects: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_seconds: i64,
    pub refresh_expiration_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub ethereum_rpc_url: String,
    pub bitcoin_rpc_url: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub log_level: String,
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("CRYPTOTRADE"))
            .build()?;

        // Set defaults
        settings.set_default("server.host", "0.0.0.0")?;
        settings.set_default("server.port", 8080)?;
        settings.set_default("server.cors_origins", vec!["http://localhost:3000"])?;

        settings.set_default("database.max_connections", 20)?;
        settings.set_default("database.min_connections", 5)?;
        settings.set_default("database.connect_timeout", 30)?;
        settings.set_default("database.idle_timeout", 600)?;

        settings.set_default("redis.max_connections", 10)?;
        settings.set_default("redis.connect_timeout", 30)?;

        settings.set_default("nats.max_reconnects", 10)?;

        settings.set_default("jwt.expiration_seconds", 3600)?; // 1 hour
        settings.set_default("jwt.refresh_expiration_days", 30)?; // 30 days

        settings.set_default("app.name", "CryptoTrade Exchange")?;
        settings.set_default("app.version", "1.0.0")?;
        settings.set_default("app.environment", "development")?;
        settings.set_default("app.log_level", "info")?;
        settings.set_default("app.metrics_enabled", true)?;
        settings.set_default("app.tracing_enabled", true)?;

        // Required environment variables
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/cryptotrade".to_string());
        settings.set("database.url", database_url)?;

        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());
        settings.set("redis.url", redis_url)?;

        let nats_url = env::var("NATS_URL")
            .unwrap_or_else(|_| "nats://localhost:4222".to_string());
        settings.set("nats.url", nats_url)?;

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
        settings.set("jwt.secret", jwt_secret)?;

        let ethereum_rpc = env::var("ETHEREUM_RPC_URL")
            .unwrap_or_else(|_| "https://mainnet.infura.io/v3/YOUR_PROJECT_ID".to_string());
        settings.set("blockchain.ethereum_rpc_url", ethereum_rpc)?;

        let bitcoin_rpc = env::var("BITCOIN_RPC_URL")
            .unwrap_or_else(|_| "http://localhost:8332".to_string());
        settings.set("blockchain.bitcoin_rpc_url", bitcoin_rpc)?;

        let private_key = env::var("BLOCKCHAIN_PRIVATE_KEY")
            .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000000000000000000000000000".to_string());
        settings.set("blockchain.private_key", private_key)?;

        settings.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.app.name, "CryptoTrade Exchange");
    }
}
