export interface WireguardPeer {
  id: number;
  interface_id: number;
  interface_name: string | null;
  employee_id: number | null;
  employee_name: string | null;
  name: string;
  public_key: string;
  allowed_ips: string;
  client_address: string;
  client_dns: string | null;
  endpoint_address: string | null;
  endpoint_port: number | null;
  persistent_keepalive: number | null;
  status: string;
  last_handshake: string | null;
  rx_bytes: number | null;
  tx_bytes: number | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface WireguardPeerListResponse {
  peers: WireguardPeer[];
  total: number;
  page: number;
  per_page: number;
}

export interface WireguardInterface {
  id: number;
  name: string;
  listen_port: number;
  public_key: string;
  mtu: number | null;
  is_active: boolean | null;
  mikrotik_id: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateWireguardPeerData {
  interface_id: number;
  employee_id?: number;
  name: string;
  allowed_ips?: string;
  client_address: string;
  client_dns?: string;
  endpoint_address?: string;
  endpoint_port?: number;
  persistent_keepalive?: number;
  notes?: string;
}

export interface UpdateWireguardPeerData {
  employee_id?: number;
  name?: string;
  allowed_ips?: string;
  client_address?: string;
  client_dns?: string;
  endpoint_address?: string;
  endpoint_port?: number;
  persistent_keepalive?: number;
  status?: string;
  notes?: string;
}

export interface WireguardConfigResponse {
  config: string;
  qr_code: string | null;
}

export interface MikrotikInterface {
  id: string;
  name: string;
  listen_port: number;
  public_key: string;
  mtu: number | null;
  disabled: boolean | null;
}

export interface MikrotikPeer {
  id: string;
  interface: string;
  public_key: string;
  allowed_address: string;
  endpoint_address: string | null;
  endpoint_port: number | null;
  current_endpoint_address: string | null;
  current_endpoint_port: number | null;
  last_handshake: string | null;
  rx: number | null;
  tx: number | null;
  comment: string | null;
  disabled: boolean | null;
}

export const useWireguard = () => {
  const { $api } = useNuxtApp();

  // ============================================================================
  // Peers
  // ============================================================================

  const fetchPeers = async (params?: {
    page?: number;
    per_page?: number;
    interface_id?: number;
    employee_id?: number;
    status?: string;
  }) => {
    return await $api<WireguardPeerListResponse>("/wireguard/peers", {
      params,
    });
  };

  const fetchPeer = async (id: number) => {
    return await $api<WireguardPeer>(`/wireguard/peers/${id}`);
  };

  const createPeer = async (data: CreateWireguardPeerData) => {
    return await $api<WireguardPeer>("/wireguard/peers", {
      method: "POST",
      body: data,
    });
  };

  const updatePeer = async (id: number, data: UpdateWireguardPeerData) => {
    return await $api<WireguardPeer>(`/wireguard/peers/${id}`, {
      method: "PUT",
      body: data,
    });
  };

  const deletePeer = async (id: number) => {
    return await $api(`/wireguard/peers/${id}`, {
      method: "DELETE",
    });
  };

  const downloadConfig = async (id: number) => {
    return await $api<WireguardConfigResponse>(
      `/wireguard/peers/${id}/config`,
    );
  };

  const syncPeerStats = async (id: number) => {
    return await $api<WireguardPeer>(`/wireguard/peers/${id}/sync`, {
      method: "POST",
    });
  };

  const syncAllPeers = async () => {
    return await $api<WireguardPeerListResponse>("/wireguard/peers/sync-all", {
      method: "POST",
    });
  };

  // ============================================================================
  // Interfaces
  // ============================================================================

  const fetchInterfaces = async () => {
    return await $api<WireguardInterface[]>("/wireguard/peers/interfaces");
  };

  const syncInterfaces = async () => {
    return await $api<WireguardInterface[]>("/wireguard/peers/interfaces/sync", {
      method: "POST",
    });
  };

  const importPeersFromMikrotik = async () => {
    return await $api<WireguardPeerListResponse>("/wireguard/peers/import-from-mikrotik", {
      method: "POST",
    });
  };

  // ============================================================================
  // MikroTik Integration
  // ============================================================================

  const fetchMikrotikInterfaces = async () => {
    return await $api<MikrotikInterface[]>(
      "/wireguard/peers/mikrotik/interfaces",
    );
  };

  const fetchMikrotikPeers = async () => {
    return await $api<MikrotikPeer[]>("/wireguard/peers/mikrotik/peers");
  };

  const fetchMikrotikInterface = async (id: string) => {
    return await $api<MikrotikInterface>(
      `/wireguard/peers/mikrotik/interfaces/${id}`,
    );
  };

  // ============================================================================
  // Utilities
  // ============================================================================

  const formatBytes = (bytes: number | null): string => {
    if (!bytes) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
  };

  const formatLastHandshake = (timestamp: string | null): string => {
    if (!timestamp) return "Никогда";
    // Добавляем 'Z' чтобы указать, что время в UTC
    const date = new Date(timestamp + 'Z');
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 0) return `${days}д назад`;
    if (hours > 0) return `${hours}ч назад`;
    if (minutes > 0) return `${minutes}м назад`;
    return "Только что";
  };

  const getStatusColor = (status: string): string => {
    const colors: Record<string, string> = {
      active: "green",
      disabled: "gray",
      expired: "red",
      revoked: "red",
    };
    return colors[status] || "gray";
  };

  const getStatusLabel = (status: string): string => {
    const labels: Record<string, string> = {
      active: "Активен",
      disabled: "Отключен",
      expired: "Истек",
      revoked: "Отозван",
    };
    return labels[status] || status;
  };

  return {
    fetchPeers,
    fetchPeer,
    createPeer,
    updatePeer,
    deletePeer,
    downloadConfig,
    syncPeerStats,
    syncAllPeers,
    importPeersFromMikrotik,
    fetchInterfaces,
    syncInterfaces,
    fetchMikrotikInterfaces,
    fetchMikrotikPeers,
    fetchMikrotikInterface,
    formatBytes,
    formatLastHandshake,
    getStatusColor,
    getStatusLabel,
  };
};
