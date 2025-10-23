use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::config::MikrotikConfig;
use crate::utils::error::AppError;

// Больше не нужны helper функции для десериализации, 
// так как мы работаем напрямую с бинарным протоколом

// ============================================================================
// MikroTik API Client (Binary Protocol on port 8728)
// ============================================================================

#[derive(Clone)]
pub struct MikrotikClient {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl MikrotikClient {
    pub fn new(config: &MikrotikConfig) -> Result<Self, AppError> {
        Ok(Self {
            host: config.host.clone(),
            port: config.port,
            username: config.user.clone(),
            password: config.password.clone(),
        })
    }

    /// Подключение к MikroTik и выполнение команды
    async fn execute_command(&self, command: &str, attributes: Vec<(&str, &str)>) -> Result<Vec<HashMap<String, String>>, AppError> {
        let addr = format!("{}:{}", self.host, self.port);
        
        let mut stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to connect to MikroTik: {}", e)))?;

        // Логин
        self.login(&mut stream).await?;

        // Отправляем команду
        let mut words = vec![command.to_string()];
        for (key, value) in attributes {
            words.push(format!("={}={}", key, value));
        }
        
        Self::send_sentence(&mut stream, &words).await?;

        // Читаем ответ
        let response = Self::read_response(&mut stream).await?;

        Ok(response)
    }

    /// Аутентификация на MikroTik
    async fn login(&self, stream: &mut TcpStream) -> Result<(), AppError> {
        // Отправляем команду логина
        Self::send_sentence(stream, &["/login".to_string()]).await?;
        
        // Читаем challenge (в новых версиях RouterOS >= 6.43 используется plain password)
        let _response = Self::read_response(stream).await?;

        // Отправляем credentials
        Self::send_sentence(stream, &[
            "/login".to_string(),
            format!("=name={}", self.username),
            format!("=password={}", self.password),
        ]).await?;

        // Проверяем результат
        let login_response = Self::read_response(stream).await?;
        
        if login_response.is_empty() || login_response[0].get("!done").is_none() {
            return Err(AppError::InternalError("Login failed".to_string()));
        }

        Ok(())
    }

    /// Отправка sentence (набора слов) в MikroTik API
    async fn send_sentence(stream: &mut TcpStream, words: &[String]) -> Result<(), AppError> {
        for word in words {
            Self::send_word(stream, word).await?;
        }
        // Пустое слово означает конец sentence
        Self::send_word(stream, "").await?;
        Ok(())
    }

    /// Отправка одного слова
    async fn send_word(stream: &mut TcpStream, word: &str) -> Result<(), AppError> {
        let bytes = word.as_bytes();
        let len = bytes.len();

        // Кодируем длину
        let len_bytes = Self::encode_length(len);
        stream.write_all(&len_bytes).await
            .map_err(|e| AppError::InternalError(format!("Failed to write length: {}", e)))?;

        // Отправляем данные
        if len > 0 {
            stream.write_all(bytes).await
                .map_err(|e| AppError::InternalError(format!("Failed to write word: {}", e)))?;
        }

        Ok(())
    }

    /// Кодирование длины по протоколу MikroTik
    fn encode_length(len: usize) -> Vec<u8> {
        if len < 0x80 {
            vec![len as u8]
        } else if len < 0x4000 {
            vec![((len >> 8) | 0x80) as u8, (len & 0xFF) as u8]
        } else if len < 0x200000 {
            vec![
                ((len >> 16) | 0xC0) as u8,
                ((len >> 8) & 0xFF) as u8,
                (len & 0xFF) as u8,
            ]
        } else if len < 0x10000000 {
            vec![
                ((len >> 24) | 0xE0) as u8,
                ((len >> 16) & 0xFF) as u8,
                ((len >> 8) & 0xFF) as u8,
                (len & 0xFF) as u8,
            ]
        } else {
            vec![
                0xF0,
                ((len >> 24) & 0xFF) as u8,
                ((len >> 16) & 0xFF) as u8,
                ((len >> 8) & 0xFF) as u8,
                (len & 0xFF) as u8,
            ]
        }
    }

    /// Чтение ответа от MikroTik
    async fn read_response(stream: &mut TcpStream) -> Result<Vec<HashMap<String, String>>, AppError> {
        let mut results = Vec::new();
        let mut current_reply = HashMap::new();

        loop {
            let word = Self::read_word(stream).await?;
            
            if word.is_empty() {
                // Конец sentence
                if !current_reply.is_empty() {
                    results.push(current_reply.clone());
                    current_reply.clear();
                }
                
                // Проверяем, закончился ли ответ
                if results.iter().any(|r| r.contains_key("!done")) {
                    break;
                }
            } else if word.starts_with("!done") {
                current_reply.insert("!done".to_string(), "".to_string());
            } else if word.starts_with("!re") {
                if !current_reply.is_empty() {
                    results.push(current_reply.clone());
                    current_reply.clear();
                }
                current_reply.insert("!re".to_string(), "".to_string());
            } else if word.starts_with("!trap") {
                current_reply.insert("!trap".to_string(), "".to_string());
            } else if word.starts_with("=") {
                // Атрибут
                if let Some(eq_pos) = word[1..].find('=') {
                    let key = &word[1..eq_pos + 1];
                    let value = &word[eq_pos + 2..];
                    current_reply.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Проверяем на ошибки trap
        for result in &results {
            if result.contains_key("!trap") {
                let message = result.get("message").cloned().unwrap_or_else(|| "Unknown error".to_string());
                let category = result.get("category").cloned().unwrap_or_else(|| "0".to_string());
                return Err(AppError::InternalError(format!("MikroTik error (category {}): {}", category, message)));
            }
        }
        
        Ok(results)
    }

    /// Чтение одного слова
    async fn read_word(stream: &mut TcpStream) -> Result<String, AppError> {
        let len = Self::read_length(stream).await?;
        
        if len == 0 {
            return Ok(String::new());
        }

        let mut buffer = vec![0u8; len];
        stream.read_exact(&mut buffer).await
            .map_err(|e| AppError::InternalError(format!("Failed to read word: {}", e)))?;

        String::from_utf8(buffer)
            .map_err(|e| AppError::InternalError(format!("Invalid UTF-8: {}", e)))
    }

    /// Чтение длины по протоколу MikroTik
    async fn read_length(stream: &mut TcpStream) -> Result<usize, AppError> {
        let mut first_byte = [0u8; 1];
        stream.read_exact(&mut first_byte).await
            .map_err(|e| AppError::InternalError(format!("Failed to read length: {}", e)))?;

        let first = first_byte[0];

        if first & 0x80 == 0 {
            Ok(first as usize)
        } else if first & 0xC0 == 0x80 {
            let mut second = [0u8; 1];
            stream.read_exact(&mut second).await
                .map_err(|e| AppError::InternalError(format!("Failed to read length: {}", e)))?;
            Ok((((first & 0x3F) as usize) << 8) | (second[0] as usize))
        } else if first & 0xE0 == 0xC0 {
            let mut bytes = [0u8; 2];
            stream.read_exact(&mut bytes).await
                .map_err(|e| AppError::InternalError(format!("Failed to read length: {}", e)))?;
            Ok((((first & 0x1F) as usize) << 16) | ((bytes[0] as usize) << 8) | (bytes[1] as usize))
        } else if first & 0xF0 == 0xE0 {
            let mut bytes = [0u8; 3];
            stream.read_exact(&mut bytes).await
                .map_err(|e| AppError::InternalError(format!("Failed to read length: {}", e)))?;
            Ok((((first & 0x0F) as usize) << 24) | ((bytes[0] as usize) << 16) | ((bytes[1] as usize) << 8) | (bytes[2] as usize))
        } else {
            let mut bytes = [0u8; 4];
            stream.read_exact(&mut bytes).await
                .map_err(|e| AppError::InternalError(format!("Failed to read length: {}", e)))?;
            Ok(((bytes[0] as usize) << 24) | ((bytes[1] as usize) << 16) | ((bytes[2] as usize) << 8) | (bytes[3] as usize))
        }
    }

    // ========================================================================
    // WireGuard Interface Operations
    // ========================================================================

    /// Получить список WireGuard интерфейсов
    pub async fn get_wireguard_interfaces(&self) -> Result<Vec<MikrotikWireguardInterface>, AppError> {
        tracing::info!("Requesting WireGuard interfaces from MikroTik via API port {}", self.port);
        
        let response = self.execute_command("/interface/wireguard/print", vec![]).await?;
        
        let mut interfaces = Vec::new();
        for item in response {
            if item.contains_key("!re") {
                let interface = MikrotikWireguardInterface {
                    id: item.get(".id").cloned().unwrap_or_default(),
                    name: item.get("name").cloned().unwrap_or_default(),
                    listen_port: item.get("listen-port").and_then(|s| s.parse().ok()).unwrap_or(0),
                    public_key: item.get("public-key").cloned().unwrap_or_default(),
                    private_key: item.get("private-key").cloned(),
                    mtu: item.get("mtu").and_then(|s| s.parse().ok()),
                    disabled: item.get("disabled").map(|s| s == "true"),
                };
                interfaces.push(interface);
            }
        }

        tracing::info!("Successfully retrieved {} WireGuard interfaces", interfaces.len());
        Ok(interfaces)
    }

    /// Получить информацию о конкретном WireGuard интерфейсе
    pub async fn get_wireguard_interface(&self, interface_id: &str) -> Result<MikrotikWireguardInterface, AppError> {
        let response = self.execute_command("/interface/wireguard/print", vec![(".id", interface_id)]).await?;
        
        for item in response {
            if item.contains_key("!re") {
                return Ok(MikrotikWireguardInterface {
                    id: item.get(".id").cloned().unwrap_or_default(),
                    name: item.get("name").cloned().unwrap_or_default(),
                    listen_port: item.get("listen-port").and_then(|s| s.parse().ok()).unwrap_or(0),
                    public_key: item.get("public-key").cloned().unwrap_or_default(),
                    private_key: item.get("private-key").cloned(),
                    mtu: item.get("mtu").and_then(|s| s.parse().ok()),
                    disabled: item.get("disabled").map(|s| s == "true"),
                });
            }
        }

        Err(AppError::NotFound("WireGuard interface not found".to_string()))
    }

    // ========================================================================
    // WireGuard Peer Operations
    // ========================================================================

    /// Получить список WireGuard пиров
    pub async fn get_wireguard_peers(&self) -> Result<Vec<MikrotikWireguardPeer>, AppError> {
        tracing::info!("Requesting WireGuard peers from MikroTik via API port {}", self.port);
        
        let response = self.execute_command("/interface/wireguard/peers/print", vec![]).await?;
        
        let mut peers = Vec::new();
        for item in response {
            if item.contains_key("!re") {
                let peer = MikrotikWireguardPeer {
                    id: item.get(".id").cloned().unwrap_or_default(),
                    interface: item.get("interface").cloned().unwrap_or_default(),
                    public_key: item.get("public-key").cloned().unwrap_or_default(),
                    preshared_key: item.get("preshared-key").cloned(),
                    allowed_address: item.get("allowed-address").cloned().unwrap_or_default(),
                    endpoint_address: item.get("endpoint-address").cloned(),
                    endpoint_port: item.get("endpoint-port").and_then(|s| s.parse().ok()),
                    persistent_keepalive: item.get("persistent-keepalive").cloned(),
                    comment: item.get("comment").cloned(),
                    disabled: item.get("disabled").map(|s| s == "true"),
                    current_endpoint_address: item.get("current-endpoint-address").cloned(),
                    current_endpoint_port: item.get("current-endpoint-port").and_then(|s| s.parse().ok()),
                    last_handshake: item.get("last-handshake").cloned(),
                    rx: item.get("rx").and_then(|s| s.parse().ok()),
                    tx: item.get("tx").and_then(|s| s.parse().ok()),
                };
                peers.push(peer);
            }
        }

        tracing::info!("Successfully retrieved {} WireGuard peers", peers.len());
        Ok(peers)
    }

    /// Создать новый WireGuard пир
    pub async fn create_wireguard_peer(&self, peer: CreateMikrotikPeerRequest) -> Result<MikrotikWireguardPeer, AppError> {
        tracing::info!("Creating WireGuard peer in MikroTik");
        tracing::debug!("Peer data: {:?}", peer);
        
        // Создаем строки заранее, чтобы они жили достаточно долго
        let port_str = peer.endpoint_port.map(|p| p.to_string());
        
        // Создаем строки для числовых полей
        let client_port_str = peer.client_listen_port.map(|p| p.to_string());
        
        let mut attributes = vec![
            ("interface", peer.interface.as_str()),
            ("public-key", peer.public_key.as_str()),
            ("allowed-address", peer.allowed_address.as_str()),
        ];

        if let Some(ref priv_key) = peer.private_key {
            attributes.push(("private-key", priv_key.as_str()));
        }
        if let Some(ref psk) = peer.preshared_key {
            attributes.push(("preshared-key", psk.as_str()));
        }
        if let Some(ref addr) = peer.endpoint_address {
            attributes.push(("endpoint-address", addr.as_str()));
        }
        if let Some(ref port) = port_str {
            attributes.push(("endpoint-port", port.as_str()));
        }
        if let Some(ref keepalive) = peer.persistent_keepalive {
            attributes.push(("persistent-keepalive", keepalive.as_str()));
        }
        // Клиентские поля для генерации конфига
        if let Some(ref client_addr) = peer.client_address {
            attributes.push(("client-address", client_addr.as_str()));
        }
        if let Some(ref client_dns) = peer.client_dns {
            attributes.push(("client-dns", client_dns.as_str()));
        }
        if let Some(ref client_endpoint) = peer.client_endpoint {
            attributes.push(("client-endpoint", client_endpoint.as_str()));
        }
        if let Some(ref client_keepalive) = peer.client_keepalive {
            attributes.push(("client-keepalive", client_keepalive.as_str()));
        }
        if let Some(ref client_port) = client_port_str {
            attributes.push(("client-listen-port", client_port.as_str()));
        }
        if let Some(ref comment) = peer.comment {
            attributes.push(("comment", comment.as_str()));
        }

        let response = self.execute_command("/interface/wireguard/peers/add", attributes).await?;
        
        // Получаем ID созданного пира
        let peer_id = response.iter()
            .find(|r| r.contains_key("!done"))
            .and_then(|r| r.get("ret"))
            .ok_or_else(|| AppError::InternalError("Failed to get created peer ID".to_string()))?
            .clone();

        // Получаем все пиры и находим созданный по ID
        let all_peers_response = self.execute_command("/interface/wireguard/peers/print", vec![]).await?;
        
        for item in all_peers_response {
            if item.contains_key("!re") && item.get(".id") == Some(&peer_id) {
                return Ok(MikrotikWireguardPeer {
                    id: item.get(".id").cloned().unwrap_or_default(),
                    interface: item.get("interface").cloned().unwrap_or_default(),
                    public_key: item.get("public-key").cloned().unwrap_or_default(),
                    preshared_key: item.get("preshared-key").cloned(),
                    allowed_address: item.get("allowed-address").cloned().unwrap_or_default(),
                    endpoint_address: item.get("endpoint-address").cloned(),
                    endpoint_port: item.get("endpoint-port").and_then(|s| s.parse().ok()),
                    persistent_keepalive: item.get("persistent-keepalive").cloned(),
                    comment: item.get("comment").cloned(),
                    disabled: item.get("disabled").map(|s| s == "true"),
                    current_endpoint_address: None,
                    current_endpoint_port: None,
                    last_handshake: None,
                    rx: None,
                    tx: None,
                });
            }
        }

        Err(AppError::InternalError(format!("Failed to retrieve created peer with ID: {}", peer_id)))
    }

    /// Обновить WireGuard пир
    pub async fn update_wireguard_peer(&self, peer_id: &str, peer: UpdateMikrotikPeerRequest) -> Result<MikrotikWireguardPeer, AppError> {
        // Создаем строки заранее, чтобы они жили достаточно долго
        let port_str = peer.endpoint_port.map(|p| p.to_string());
        let disabled_str = peer.disabled.map(|d| if d { String::from("yes") } else { String::from("no") });
        
        let mut attributes = vec![(".id", peer_id)];

        if let Some(ref addr) = peer.allowed_address {
            attributes.push(("allowed-address", addr.as_str()));
        }
        if let Some(ref endpoint) = peer.endpoint_address {
            attributes.push(("endpoint-address", endpoint.as_str()));
        }
        if let Some(ref port) = port_str {
            attributes.push(("endpoint-port", port.as_str()));
        }
        if let Some(ref keepalive) = peer.persistent_keepalive {
            attributes.push(("persistent-keepalive", keepalive.as_str()));
        }
        if let Some(ref comment) = peer.comment {
            attributes.push(("comment", comment.as_str()));
        }
        if let Some(ref disabled) = disabled_str {
            attributes.push(("disabled", disabled.as_str()));
        }

        self.execute_command("/interface/wireguard/peers/set", attributes).await?;

        // Получаем все пиры и находим обновленный по ID
        let all_peers_response = self.execute_command("/interface/wireguard/peers/print", vec![]).await?;
        
        for item in all_peers_response {
            if item.contains_key("!re") && item.get(".id") == Some(&peer_id.to_string()) {
                return Ok(MikrotikWireguardPeer {
                    id: item.get(".id").cloned().unwrap_or_default(),
                    interface: item.get("interface").cloned().unwrap_or_default(),
                    public_key: item.get("public-key").cloned().unwrap_or_default(),
                    preshared_key: item.get("preshared-key").cloned(),
                    allowed_address: item.get("allowed-address").cloned().unwrap_or_default(),
                    endpoint_address: item.get("endpoint-address").cloned(),
                    endpoint_port: item.get("endpoint-port").and_then(|s| s.parse().ok()),
                    persistent_keepalive: item.get("persistent-keepalive").cloned(),
                    comment: item.get("comment").cloned(),
                    disabled: item.get("disabled").map(|s| s == "true"),
                    current_endpoint_address: item.get("current-endpoint-address").cloned(),
                    current_endpoint_port: item.get("current-endpoint-port").and_then(|s| s.parse().ok()),
                    last_handshake: item.get("last-handshake").cloned(),
                    rx: item.get("rx").and_then(|s| s.parse().ok()),
                    tx: item.get("tx").and_then(|s| s.parse().ok()),
                });
            }
        }

        Err(AppError::NotFound(format!("WireGuard peer with ID {} not found", peer_id)))
    }

    /// Удалить WireGuard пир
    pub async fn delete_wireguard_peer(&self, peer_id: &str) -> Result<(), AppError> {
        self.execute_command("/interface/wireguard/peers/remove", vec![(".id", peer_id)]).await?;
        Ok(())
    }

    /// Получить статистику пира
    pub async fn get_peer_stats(&self, peer_id: &str) -> Result<MikrotikPeerStats, AppError> {
        let response = self.execute_command("/interface/wireguard/peers/print", vec![(".id", peer_id)]).await?;
        
        for item in response {
            if item.contains_key("!re") {
                return Ok(MikrotikPeerStats {
                    rx_bytes: item.get("rx").and_then(|s| s.parse().ok()).unwrap_or(0),
                    tx_bytes: item.get("tx").and_then(|s| s.parse().ok()).unwrap_or(0),
                    last_handshake: item.get("last-handshake").cloned(),
                });
            }
        }

        Err(AppError::NotFound("WireGuard peer not found".to_string()))
    }
}

// ============================================================================
// MikroTik API Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MikrotikWireguardInterface {
    pub id: String,
    pub name: String,
    pub listen_port: i32,
    pub public_key: String,
    pub private_key: Option<String>,
    pub mtu: Option<i32>,
    pub disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MikrotikWireguardPeer {
    pub id: String,
    pub interface: String,
    pub public_key: String,
    pub preshared_key: Option<String>,
    pub allowed_address: String,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub persistent_keepalive: Option<String>,
    pub comment: Option<String>,
    pub disabled: Option<bool>,
    pub current_endpoint_address: Option<String>,
    pub current_endpoint_port: Option<i32>,
    pub last_handshake: Option<String>,
    pub rx: Option<i64>,
    pub tx: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMikrotikPeerRequest {
    pub interface: String,
    #[serde(rename = "public-key")]
    pub public_key: String,
    #[serde(rename = "private-key", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
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
    // Поля для генерации клиентского конфига в MikroTik
    #[serde(rename = "client-address", skip_serializing_if = "Option::is_none")]
    pub client_address: Option<String>,
    #[serde(rename = "client-dns", skip_serializing_if = "Option::is_none")]
    pub client_dns: Option<String>,
    #[serde(rename = "client-endpoint", skip_serializing_if = "Option::is_none")]
    pub client_endpoint: Option<String>,
    #[serde(rename = "client-keepalive", skip_serializing_if = "Option::is_none")]
    pub client_keepalive: Option<String>,
    #[serde(rename = "client-listen-port", skip_serializing_if = "Option::is_none")]
    pub client_listen_port: Option<i32>,
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
