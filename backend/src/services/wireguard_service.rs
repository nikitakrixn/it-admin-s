use std::sync::Arc;
use uuid::Uuid;

use crate::config::database::Pool;

/// Парсит MikroTik duration формат (например "31s", "5m20s", "1h30m", "2d", "3w") в секунды
fn parse_mikrotik_duration(duration: &str) -> Option<i64> {
    let mut total_seconds = 0i64;
    let mut current_number = String::new();
    
    for ch in duration.chars() {
        if ch.is_ascii_digit() {
            current_number.push(ch);
        } else {
            if let Ok(num) = current_number.parse::<i64>() {
                total_seconds += match ch {
                    's' => num,
                    'm' => num * 60,
                    'h' => num * 3600,
                    'd' => num * 86400,
                    'w' => num * 604800,
                    _ => return None,
                };
            }
            current_number.clear();
        }
    }
    
    Some(total_seconds)
}
use crate::integrations::mikrotik::{CreateMikrotikPeerRequest, MikrotikClient, UpdateMikrotikPeerRequest};
use crate::models::wireguard::{CreateWireguardPeerRequest, NewWireguardPeer, UpdateWireguardPeer};
use crate::repositories::wireguard_repository::WireguardRepository;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct WireguardService {
    repository: WireguardRepository,
    mikrotik_client: Option<Arc<MikrotikClient>>,
}

impl WireguardService {
    pub fn new(pool: Pool, mikrotik_client: Option<Arc<MikrotikClient>>) -> Self {
        Self {
            repository: WireguardRepository::new(pool),
            mikrotik_client,
        }
    }

    /// Создать новый WireGuard пир
    pub async fn create_peer(
        &self,
        request: CreateWireguardPeerRequest,
        created_by: Option<Uuid>,
    ) -> Result<i32, AppError> {
        // Получаем интерфейс для определения имени
        let interface = self.repository.get_interface_by_id(request.interface_id).await?;

        // Генерируем ключи на бэкенде
        let (public_key, private_key) = Self::generate_wireguard_keys();
        
        // Создаем пир в MikroTik если клиент доступен
        let mikrotik_peer_id = if let Some(client) = &self.mikrotik_client {
            let mikrotik_request = CreateMikrotikPeerRequest {
                interface: interface.name.clone(),
                public_key: public_key.clone(), // Публичный ключ пира
                private_key: Some(private_key.clone()), // Приватный ключ для генерации конфига в MikroTik
                preshared_key: None,
                allowed_address: request.allowed_ips.clone().unwrap_or_else(|| "0.0.0.0/0,::/0".to_string()), // Allowed address для роутера
                endpoint_address: request.endpoint_address.clone(),
                endpoint_port: request.endpoint_port,
                persistent_keepalive: request.persistent_keepalive.map(|k| format!("{}s", k)),
                // Клиентские поля для генерации конфига в MikroTik
                client_address: Some(request.client_address.clone()),
                client_dns: request.client_dns.clone(),
                client_endpoint: request.endpoint_address.clone(),
                client_keepalive: request.persistent_keepalive.map(|k| format!("{}s", k)),
                client_listen_port: request.endpoint_port,
                comment: Some(request.name.clone()),
            };

            match client.create_wireguard_peer(mikrotik_request).await {
                Ok(peer) => {
                    tracing::info!("Successfully created peer in MikroTik with ID: {}", peer.id);
                    Some(peer.id.clone())
                },
                Err(e) => {
                    tracing::error!("Failed to create peer in MikroTik: {}", e);
                    return Err(e);
                }
            }
        } else {
            None
        };

        // Сохраняем пир в БД с приватным ключом
        let new_peer = NewWireguardPeer {
            interface_id: request.interface_id,
            employee_id: request.employee_id,
            name: request.name,
            public_key,
            private_key: Some(private_key),
            preshared_key: None,
            allowed_ips: request.allowed_ips.unwrap_or_else(|| "0.0.0.0/0,::/0".to_string()),
            client_address: request.client_address,
            client_dns: request.client_dns,
            endpoint_address: request.endpoint_address,
            endpoint_port: request.endpoint_port,
            persistent_keepalive: request.persistent_keepalive,
            mikrotik_peer_id,
            status: "active".to_string(),
            notes: request.notes,
            created_by,
        };

        let peer = self.repository.create_peer(new_peer).await?;
        Ok(peer.id)
    }

    /// Обновить WireGuard пир
    pub async fn update_peer(
        &self,
        peer_id: i32,
        update_data: UpdateWireguardPeer,
    ) -> Result<(), AppError> {
        tracing::info!("Updating peer {}", peer_id);
        tracing::debug!("Update data: {:?}", update_data);
        
        // Получаем текущий пир
        let (peer, _, _, _) = self.repository.get_peer_by_id(peer_id).await?;

        // Обновляем в MikroTik если есть mikrotik_peer_id
        if let (Some(client), Some(mikrotik_id)) = (&self.mikrotik_client, &peer.mikrotik_peer_id) {
            tracing::info!("Updating peer in MikroTik with ID: {}", mikrotik_id);
            
            let mikrotik_update = UpdateMikrotikPeerRequest {
                allowed_address: update_data.allowed_ips.clone(),
                endpoint_address: update_data.endpoint_address.clone(),
                endpoint_port: update_data.endpoint_port,
                persistent_keepalive: update_data.persistent_keepalive.map(|k| format!("{}s", k)),
                comment: update_data.name.clone(),
                disabled: update_data.status.as_ref().map(|s| s == "disabled"),
            };

            match client.update_wireguard_peer(mikrotik_id, mikrotik_update).await {
                Ok(_) => tracing::info!("Successfully updated peer in MikroTik"),
                Err(e) => {
                    tracing::error!("Failed to update peer in MikroTik: {}", e);
                    return Err(e);
                }
            }
        }

        // Обновляем в БД
        tracing::info!("Updating peer in database");
        self.repository.update_peer(peer_id, update_data).await?;
        tracing::info!("Successfully updated peer {}", peer_id);
        Ok(())
    }

    /// Удалить WireGuard пир
    pub async fn delete_peer(&self, peer_id: i32) -> Result<(), AppError> {
        // Получаем пир
        let (peer, _, _, _) = self.repository.get_peer_by_id(peer_id).await?;

        // Удаляем из MikroTik если есть mikrotik_peer_id
        if let (Some(client), Some(mikrotik_id)) = (&self.mikrotik_client, &peer.mikrotik_peer_id) {
            if let Err(e) = client.delete_wireguard_peer(mikrotik_id).await {
                tracing::warn!("Failed to delete peer from MikroTik: {}", e);
                // Продолжаем удаление из БД даже если не удалось удалить из MikroTik
            }
        }

        // Удаляем из БД
        self.repository.delete_peer(peer_id).await?;
        Ok(())
    }

    /// Синхронизировать статистику пира с MikroTik
    pub async fn sync_peer_stats(&self, peer_id: i32) -> Result<(), AppError> {
        let (peer, _, _, _) = self.repository.get_peer_by_id(peer_id).await?;

        if let (Some(client), Some(mikrotik_id)) = (&self.mikrotik_client, &peer.mikrotik_peer_id) {
            let stats = client.get_peer_stats(mikrotik_id).await?;

            tracing::debug!("Syncing stats for peer {}: rx={}, tx={}, last_handshake={:?}", 
                peer_id, stats.rx_bytes, stats.tx_bytes, stats.last_handshake);

            // Парсим last_handshake если есть
            // MikroTik возвращает относительное время: "31s", "5m20s", "1h30m", "2d", "3w"
            let last_handshake = if let Some(ref handshake_str) = stats.last_handshake {
                // Парсим относительное время и вычисляем абсолютное
                if let Some(seconds_ago) = parse_mikrotik_duration(handshake_str) {
                    let now = chrono::Utc::now().naive_utc();
                    Some(now - chrono::Duration::seconds(seconds_ago))
                } else {
                    // Пробуем распарсить как timestamp
                    chrono::NaiveDateTime::parse_from_str(handshake_str, "%Y-%m-%d %H:%M:%S").ok()
                }
            } else {
                // Если last_handshake не пришёл, но трафик изменился - значит соединение активно
                // Обновляем last_handshake на текущее время
                let traffic_changed = peer.rx_bytes != Some(stats.rx_bytes) || peer.tx_bytes != Some(stats.tx_bytes);
                if traffic_changed && (stats.rx_bytes > 0 || stats.tx_bytes > 0) {
                    tracing::debug!("Traffic changed for peer {}, updating last_handshake to now", peer_id);
                    Some(chrono::Utc::now().naive_utc())
                } else {
                    None
                }
            };

            tracing::debug!("Parsed last_handshake: {:?}", last_handshake);

            self.repository
                .update_peer_stats(peer_id, stats.rx_bytes, stats.tx_bytes, last_handshake)
                .await?;
        }

        Ok(())
    }

    /// Генерировать конфигурационный файл для клиента
    pub fn generate_client_config(
        &self,
        peer: &crate::models::wireguard::WireguardPeer,
        interface: &crate::models::wireguard::WireguardInterface,
        server_endpoint: &str,
    ) -> String {
        let private_key = peer.private_key.as_ref()
            .map(|k| k.as_str())
            .unwrap_or("[PRIVATE_KEY - Not available]");

        let dns = peer.client_dns.as_ref()
            .map(|d| format!("DNS = {}\n", d))
            .unwrap_or_default();

        format!(
            r#"[Interface]
PrivateKey = {}
Address = {}
{}
[Peer]
PublicKey = {}
Endpoint = {}:{}
AllowedIPs = {}
PersistentKeepalive = {}
"#,
            private_key,
            peer.client_address,
            dns,
            interface.public_key,
            server_endpoint,
            interface.listen_port,
            peer.allowed_ips,
            peer.persistent_keepalive.unwrap_or(25)
        )
    }

    /// Синхронизировать интерфейсы из MikroTik в БД
    pub async fn sync_interfaces(&self) -> Result<usize, AppError> {
        if let Some(client) = &self.mikrotik_client {
            let mikrotik_interfaces = client.get_wireguard_interfaces().await?;
            let mut synced_count = 0;

            for mt_iface in mikrotik_interfaces {
                // Проверяем, существует ли интерфейс в БД
                let existing = self.repository.find_interface_by_mikrotik_id(&mt_iface.id).await;

                match existing {
                    Ok(Some(_)) => {
                        // Интерфейс уже существует, можно обновить
                        tracing::debug!("Interface {} already exists in DB", mt_iface.name);
                    }
                    Ok(None) | Err(_) => {
                        // Создаем новый интерфейс
                        let new_interface = crate::models::wireguard::NewWireguardInterface {
                            name: mt_iface.name.clone(),
                            listen_port: mt_iface.listen_port,
                            public_key: mt_iface.public_key.clone(),
                            private_key: None, // MikroTik не отдает приватный ключ
                            mtu: mt_iface.mtu,
                            is_active: Some(!mt_iface.disabled.unwrap_or(false)),
                            mikrotik_id: Some(mt_iface.id.clone()),
                            notes: None,
                        };

                        match self.repository.create_interface(new_interface).await {
                            Ok(_) => {
                                synced_count += 1;
                                tracing::info!("Synced interface {} from MikroTik", mt_iface.name);
                            }
                            Err(e) => {
                                tracing::error!("Failed to sync interface {}: {}", mt_iface.name, e);
                            }
                        }
                    }
                }
            }

            Ok(synced_count)
        } else {
            Err(AppError::ValidationError("MikroTik integration is not enabled".to_string()))
        }
    }

    /// Синхронизировать пиры из MikroTik в БД
    pub async fn sync_peers_from_mikrotik(&self) -> Result<usize, AppError> {
        if let Some(client) = &self.mikrotik_client {
            tracing::info!("Starting peer sync from MikroTik");
            let mikrotik_peers = client.get_wireguard_peers().await?;
            tracing::info!("Found {} peers in MikroTik", mikrotik_peers.len());
            let mut synced_count = 0;

            for mt_peer in mikrotik_peers {
                tracing::debug!("Processing peer: interface={}, public_key={}", mt_peer.interface, &mt_peer.public_key[..16]);
                // Проверяем, существует ли пир в БД по public_key
                let existing = self.repository.find_peer_by_public_key(&mt_peer.public_key).await;

                match existing {
                    Ok(Some(_)) => {
                        // Пир уже существует
                        tracing::debug!("Peer with public key {} already exists in DB", mt_peer.public_key);
                    }
                    Ok(None) => {
                        tracing::debug!("Peer not found in DB, will create new one");
                        // Находим интерфейс по имени
                        match self.repository.list_interfaces().await {
                            Ok(interfaces) => {
                                tracing::debug!("Found {} interfaces in DB", interfaces.len());
                                if let Some(interface) = interfaces.iter().find(|i| i.name == mt_peer.interface) {
                                    tracing::debug!("Found matching interface: {} (id={})", interface.name, interface.id);
                                // Создаем новый пир
                                let new_peer = NewWireguardPeer {
                                    interface_id: interface.id,
                                    employee_id: None,
                                    name: mt_peer.comment.clone().unwrap_or_else(|| format!("Imported peer {}", &mt_peer.public_key[..8])),
                                    public_key: mt_peer.public_key.clone(),
                                    private_key: None, // MikroTik не отдает приватный ключ
                                    preshared_key: None,
                                    allowed_ips: mt_peer.allowed_address.clone(),
                                    client_address: mt_peer.allowed_address.clone(),
                                    client_dns: None,
                                    endpoint_address: mt_peer.endpoint_address.clone(),
                                    endpoint_port: mt_peer.endpoint_port,
                                    persistent_keepalive: None,
                                    mikrotik_peer_id: Some(mt_peer.id.clone()),
                                    status: if mt_peer.disabled.unwrap_or(false) { "disabled".to_string() } else { "active".to_string() },
                                    notes: Some("Импортировано из MikroTik".to_string()),
                                    created_by: None,
                                };

                                    match self.repository.create_peer(new_peer).await {
                                        Ok(_) => {
                                            synced_count += 1;
                                            tracing::info!("Synced peer {} from MikroTik", mt_peer.public_key);
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to sync peer {}: {}", mt_peer.public_key, e);
                                        }
                                    }
                                } else {
                                    tracing::warn!("Interface {} not found for peer {}", mt_peer.interface, mt_peer.public_key);
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to list interfaces: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Error checking existing peer: {}", e);
                        // Продолжаем попытку создания пира
                        match self.repository.list_interfaces().await {
                            Ok(interfaces) => {
                                tracing::debug!("Found {} interfaces in DB", interfaces.len());
                                if let Some(interface) = interfaces.iter().find(|i| i.name == mt_peer.interface) {
                                    tracing::debug!("Found matching interface: {} (id={})", interface.name, interface.id);
                                    // Создаем новый пир
                                    let new_peer = NewWireguardPeer {
                                        interface_id: interface.id,
                                        employee_id: None,
                                        name: mt_peer.comment.clone().unwrap_or_else(|| format!("Imported peer {}", &mt_peer.public_key[..8])),
                                        public_key: mt_peer.public_key.clone(),
                                        private_key: None,
                                        preshared_key: None,
                                        allowed_ips: mt_peer.allowed_address.clone(),
                                        client_address: mt_peer.allowed_address.clone(),
                                        client_dns: None,
                                        endpoint_address: mt_peer.endpoint_address.clone(),
                                        endpoint_port: mt_peer.endpoint_port,
                                        persistent_keepalive: None,
                                        mikrotik_peer_id: Some(mt_peer.id.clone()),
                                        status: if mt_peer.disabled.unwrap_or(false) { "disabled".to_string() } else { "active".to_string() },
                                        notes: Some("Импортировано из MikroTik".to_string()),
                                        created_by: None,
                                    };

                                    match self.repository.create_peer(new_peer).await {
                                        Ok(_) => {
                                            synced_count += 1;
                                            tracing::info!("Synced peer {} from MikroTik", mt_peer.public_key);
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to sync peer {}: {}", mt_peer.public_key, e);
                                        }
                                    }
                                } else {
                                    tracing::warn!("Interface {} not found for peer {}", mt_peer.interface, mt_peer.public_key);
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to list interfaces: {}", e);
                            }
                        }
                    }
                }
            }

            tracing::info!("Peer sync completed: {} peers synced", synced_count);
            Ok(synced_count)
        } else {
            Err(AppError::ValidationError("MikroTik integration is not enabled".to_string()))
        }
    }

    /// Генерация WireGuard ключей
    fn generate_wireguard_keys() -> (String, String) {
        use base64::{Engine as _, engine::general_purpose};
        use rand::rngs::OsRng;
        use x25519_dalek::{StaticSecret, PublicKey};

        // Генерируем приватный ключ
        let private_key = StaticSecret::random_from_rng(OsRng);
        let private_key_bytes = private_key.to_bytes();
        
        // Генерируем публичный ключ из приватного
        let public_key = PublicKey::from(&private_key);
        let public_key_bytes = public_key.to_bytes();

        // Кодируем в base64
        let private_key_b64 = general_purpose::STANDARD.encode(private_key_bytes);
        let public_key_b64 = general_purpose::STANDARD.encode(public_key_bytes);

        (public_key_b64, private_key_b64)
    }
}
