pub mod database;

use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
    pub ad: Option<AdConfig>,
    pub mikrotik: Option<MikrotikConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub env: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdConfig {
    pub enabled: bool,
    pub server: String,
    pub domain: String,
    pub base_dn: String,
    pub bind_dn: String,
    pub bind_password: String,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MikrotikConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config {
            app: AppConfig {
                name: env::var("APP_NAME").unwrap_or_else(|_| "IT-Admin".to_string()),
                env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
                host: env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("APP_PORT")
                    .unwrap_or_else(|_| "8000".to_string())
                    .parse()?,
                log_level: env::var("APP_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")?,
                expiration: env::var("JWT_EXPIRATION")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .unwrap_or(86400),
            },
            cors: CorsConfig {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            ad: Self::load_ad_config(),
            mikrotik: Self::load_mikrotik_config(),
        })
    }

    fn load_ad_config() -> Option<AdConfig> {
        if env::var("AD_ENABLED").unwrap_or_default() == "true" {
            Some(AdConfig {
                enabled: true,
                server: env::var("AD_SERVER").ok()?,
                domain: env::var("AD_DOMAIN").ok()?,
                base_dn: env::var("AD_BASE_DN").ok()?,
                bind_dn: env::var("AD_BIND_DN").ok()?,
                bind_password: env::var("AD_BIND_PASSWORD").ok()?,
                use_tls: env::var("AD_USE_TLS").unwrap_or_else(|_| "true".to_string()) == "true",
            })
        } else {
            None
        }
    }

    fn load_mikrotik_config() -> Option<MikrotikConfig> {
        if env::var("MIKROTIK_ENABLED").unwrap_or_default() == "true" {
            Some(MikrotikConfig {
                enabled: true,
                host: env::var("MIKROTIK_HOST").ok()?,
                port: env::var("MIKROTIK_PORT").ok()?.parse().ok()?,
                user: env::var("MIKROTIK_USER").ok()?,
                password: env::var("MIKROTIK_PASSWORD").ok()?,
            })
        } else {
            None
        }
    }
}
