use reqwest::{Client, StatusCode};
use serde::{Deserialize, Deserializer, Serialize};
use std::error::Error;
use std::time::Duration;

use crate::config::MikrotikConfig;
use crate::utils::error::AppError;

// Helper для десериализации булевых значений из строк
fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.and_then(|s| match s.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }))
}

// Helper для десериализации i32 из строк
fn deserialize_i32_from_string<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: String = String::deserialize(deserializer)?;
    s.parse::<i32>().map_err(D::Error::custom)
}

// Helper для десериализации Option<i32> из строк
fn deserialize_option_i32_from_string<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s.parse::<i32>().map(Some).map_err(D::Error::custom),
        None => Ok(None),
    }
}

// Helper для десериализации Option<i64> из строк
fn deserialize_option_i64_from_string<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s.parse::<i64>().map(Some).map_err(D::Error::custom),
        None => Ok(None),
    }
}

// ============================================================================
// MikroTik REST API Client
// ============================================================================

#[derive(Clone)]
pub struct MikrotikClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
}

impl MikrotikClient {
    pub fn new(config: &MikrotikConfig) -> Result<Self, AppError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .danger_accept_invalid_certs(true) // MikroTik часто использует self-signed сертификаты
            .min_tls_version(reqwest::tls::Version::TLS_1_0) // Поддержка старых версий TLS для MikroTik
            .use_rustls_tls() // Использовать rustls вместо native-tls для лучшей совместимости
            .build()
            .map_err(|e| AppError::InternalError(format!("Failed to create HTTP client: {}", e)))?;

        let protocol = if config.port == 443 { "https" } else { "http" };
        
        Ok(Self {
            client,
            base_url: format!("{}://{}:{}/rest", protocol, config.host, config.port),
            username: config.user.clone(),
            password: config.password.clone(),
        })
    }

    // ========================================================================
    // WireGuard Interface Operations
    // ========================================================================

    /// Получить список WireGuard интерфейсов
    pub async fn get_wireguard_interfaces(&self) -> Result<Vec<MikrotikWireguardInterface>, AppError> {
        let url = format!("{}/interface/wireguard", self.base_url);
        tracing::info!("Requesting WireGuard interfaces from MikroTik: {}", url);
        
        let response = self.client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("MikroTik API request failed: {} - URL: {}", e, url);
                if let Some(source) = e.source() {
                    tracing::error!("Error source: {}", source);
                }
                AppError::InternalError(format!("MikroTik API request failed: {} (URL: {})", e, url))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Unable to read body".to_string());
            tracing::error!("MikroTik API error: status={}, body={}", status, body);
            return Err(AppError::InternalError(format!(
                "MikroTik API error: {} - {}",
                status, body
            )));
        }

        let body = response.text().await
            .map_err(|e| {
                tracing::error!("Failed to read response body: {}", e);
                AppError::InternalError(format!("Failed to read response body: {}", e))
            })?;
        
        tracing::info!("MikroTik WireGuard interfaces response: {}", body);

        let interfaces: Vec<MikrotikWireguardInterface> = serde_json::from_str(&body)
            .map_err(|e| {
                tracing::error!("Failed to parse response: {} - Body: {}", e, body);
                AppError::InternalError(format!("Failed to parse response: {} - Body: {}", e, body))
            })?;

        tracing::info!("Successfully parsed {} WireGuard interfaces", interfaces.len());
        Ok(interfaces)
    }

    /// Получить информацию о конкретном WireGuard интерфейсе
    pub async fn get_wireguard_interface(&self, interface_id: &str) -> Result<MikrotikWireguardInterface, AppError> {
        let url = format!("{}/interface/wireguard/{}", self.base_url, interface_id);
        
        let response = self.client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("MikroTik API request failed: {}", e)))?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound("WireGuard interface not found".to_string()));
        }

        if !response.status().is_success() {
            return Err(AppError::InternalError(format!(
                "MikroTik API error: {}",
                response.status()
            )));
        }

        let interface: MikrotikWireguardInterface = response
            .json()
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to parse response: {}", e)))?;

        Ok(interface)
    }

    // ========================================================================
    // WireGuard Peer Operations
    // ========================================================================

    /// Получить список WireGuard пиров
    pub async fn get_wireguard_peers(&self) -> Result<Vec<MikrotikWireguardPeer>, AppError> {
        let url = format!("{}/interface/wireguard/peers", self.base_url);
        tracing::info!("Requesting WireGuard peers from MikroTik: {}", url);
        
        let response = self.client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("MikroTik API request failed: {} - URL: {}", e, url);
                if let Some(source) = e.source() {
                    tracing::error!("Error source: {}", source);
                }
                AppError::InternalError(format!("MikroTik API request failed: {} (URL: {})", e, url))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Unable to read body".to_string());
            tracing::error!("MikroTik API error: status={}, body={}", status, body);
            return Err(AppError::InternalError(format!(
                "MikroTik API error: {} - {}",
                status, body
            )));
        }

        let body = response.text().await
            .map_err(|e| {
                tracing::error!("Failed to read response body: {}", e);
                AppError::InternalError(format!("Failed to read response body: {}", e))
            })?;
        
        tracing::info!("MikroTik WireGuard peers response: {}", body);

        let peers: Vec<MikrotikWireguardPeer> = serde_json::from_str(&body)
            .map_err(|e| {
                tracing::error!("Failed to parse response: {} - Body: {}", e, body);
                AppError::InternalError(format!("Failed to parse response: {} - Body: {}", e, body))
            })?;

        tracing::info!("Successfully parsed {} WireGuard peers", peers.len());
        Ok(peers)
    }

    /// Создать новый WireGuard пир
    pub async fn create_wireguard_peer(&self, peer: CreateMikrotikPeerRequest) -> Result<MikrotikWireguardPeer, AppError> {
        let url = format!("{}/interface/wireguard/peers", self.base_url);
        
        tracing::info!("Creating WireGuard peer in MikroTik: {}", url);
        tracing::debug!("Peer data: {:?}", peer);
        
        let response = self.client
            .put(&url)  // MikroTik REST API использует PUT для создания
            .basic_auth(&self.username, Some(&self.password))
            .json(&peer)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to create peer: {}", e);
                AppError::InternalError(format!("MikroTik API request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("MikroTik API error: status={}, body={}", status, error_text);
            return Err(AppError::InternalError(format!(
                "MikroTik API error: {}",
                error_text
            )));
        }

        let body = response.text().await
            .map_err(|e| AppError::InternalError(format!("Failed to read response: {}", e)))?;
        
        tracing::info!("Created peer response: {}", body);

        let created_peer: MikrotikWireguardPeer = serde_json::from_str(&body)
            .map_err(|e| {
                tracing::error!("Failed to parse response: {} - Body: {}", e, body);
                AppError::InternalError(format!("Failed to parse response: {}", e))
            })?;

        Ok(created_peer)
    }

    /// Обновить WireGuard пир
    pub async fn update_wireguard_peer(&self, peer_id: &str, peer: UpdateMikrotikPeerRequest) -> Result<MikrotikWireguardPeer, AppError> {
        let url = format!("{}/interface/wireguard/peers/{}", self.base_url, peer_id);
        
        let response = self.client
            .patch(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&peer)
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("MikroTik API request failed: {}", e)))?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound("WireGuard peer not found".to_string()));
        }

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::InternalError(format!(
                "MikroTik API error: {}",
                error_text
            )));
        }

        let updated_peer: MikrotikWireguardPeer = response
            .json()
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to parse response: {}", e)))?;

        Ok(updated_peer)
    }

    /// Удалить WireGuard пир
    pub async fn delete_wireguard_peer(&self, peer_id: &str) -> Result<(), AppError> {
        let url = format!("{}/interface/wireguard/peers/{}", self.base_url, peer_id);
        
        let response = self.client
            .delete(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("MikroTik API request failed: {}", e)))?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound("WireGuard peer not found".to_string()));
        }

        if !response.status().is_success() {
            return Err(AppError::InternalError(format!(
                "MikroTik API error: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Получить статистику пира
    pub async fn get_peer_stats(&self, peer_id: &str) -> Result<MikrotikPeerStats, AppError> {
        let url = format!("{}/interface/wireguard/peers/{}", self.base_url, peer_id);
        
        let response = self.client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("MikroTik API request failed: {}", e)))?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound("WireGuard peer not found".to_string()));
        }

        if !response.status().is_success() {
            return Err(AppError::InternalError(format!(
                "MikroTik API error: {}",
                response.status()
            )));
        }

        let peer: MikrotikWireguardPeer = response
            .json()
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to parse response: {}", e)))?;

        Ok(MikrotikPeerStats {
            rx_bytes: peer.rx.unwrap_or(0),
            tx_bytes: peer.tx.unwrap_or(0),
            last_handshake: peer.last_handshake,
        })
    }
}

// ============================================================================
// MikroTik API Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MikrotikWireguardInterface {
    #[serde(rename = ".id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "listen-port", deserialize_with = "deserialize_i32_from_string")]
    pub listen_port: i32,
    #[serde(rename = "public-key")]
    pub public_key: String,
    #[serde(rename = "private-key")]
    pub private_key: Option<String>,
    #[serde(deserialize_with = "deserialize_option_i32_from_string", default)]
    pub mtu: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool_from_string", default)]
    pub disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MikrotikWireguardPeer {
    #[serde(rename = ".id")]
    pub id: String,
    pub interface: String,
    #[serde(rename = "public-key")]
    pub public_key: String,
    #[serde(rename = "preshared-key")]
    pub preshared_key: Option<String>,
    #[serde(rename = "allowed-address")]
    pub allowed_address: String,
    #[serde(rename = "endpoint-address")]
    pub endpoint_address: Option<String>,
    #[serde(rename = "endpoint-port", deserialize_with = "deserialize_option_i32_from_string", default)]
    pub endpoint_port: Option<i32>,
    #[serde(rename = "persistent-keepalive")]
    pub persistent_keepalive: Option<String>,
    pub comment: Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_string", default)]
    pub disabled: Option<bool>,
    #[serde(rename = "current-endpoint-address")]
    pub current_endpoint_address: Option<String>,
    #[serde(rename = "current-endpoint-port", deserialize_with = "deserialize_option_i32_from_string", default)]
    pub current_endpoint_port: Option<i32>,
    #[serde(rename = "last-handshake")]
    pub last_handshake: Option<String>,
    #[serde(deserialize_with = "deserialize_option_i64_from_string", default)]
    pub rx: Option<i64>,
    #[serde(deserialize_with = "deserialize_option_i64_from_string", default)]
    pub tx: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMikrotikPeerRequest {
    pub interface: String,
    #[serde(rename = "private-key")]
    pub private_key: String,
    #[serde(rename = "preshared-key", skip_serializing_if = "Option::is_none")]
    pub preshared_key: Option<String>,
    #[serde(rename = "allowed-address")]
    pub allowed_address: String,
    #[serde(rename = "endpoint-address", skip_serializing_if = "Option::is_none")]
    pub endpoint_address: Option<String>,
    #[serde(rename = "endpoint-port", skip_serializing_if = "Option::is_none")]
    pub endpoint_port: Option<i32>,
    #[serde(rename = "persistent-keepalive", skip_serializing_if = "Option::is_none")]
    pub persistent_keepalive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMikrotikPeerRequest {
    #[serde(rename = "allowed-address", skip_serializing_if = "Option::is_none")]
    pub allowed_address: Option<String>,
    #[serde(rename = "endpoint-address", skip_serializing_if = "Option::is_none")]
    pub endpoint_address: Option<String>,
    #[serde(rename = "endpoint-port", skip_serializing_if = "Option::is_none")]
    pub endpoint_port: Option<i32>,
    #[serde(rename = "persistent-keepalive", skip_serializing_if = "Option::is_none")]
    pub persistent_keepalive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MikrotikPeerStats {
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub last_handshake: Option<String>,
}
