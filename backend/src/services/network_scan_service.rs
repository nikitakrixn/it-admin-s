use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use futures::stream::{self, StreamExt};
use serde::Serialize;

/// Найденный при сканировании узел.
#[derive(Debug, Clone, Serialize)]
pub struct ScannedHost {
    pub ip: String,
    pub open_ports: Vec<u16>,
}

/// Проверяет доступность узлов диапазона TCP-подключением к указанным портам.
///
/// Это не ICMP и не SYN-скан: приложение просто пытается установить обычное
/// TCP-соединение и считает узел живым, если хотя бы один порт ответил.
/// Такой вариант работает без прав администратора и не требует сторонних утилит.
pub async fn scan_range(
    start: Ipv4Addr,
    end: Ipv4Addr,
    ports: &[u16],
    timeout: Duration,
    concurrency: usize,
) -> Vec<ScannedHost> {
    let start = u32::from(start);
    let end = u32::from(end);

    let targets: Vec<(Ipv4Addr, u16)> = (start..=end)
        .flat_map(|addr| {
            let ip = Ipv4Addr::from(addr);
            ports.iter().map(move |port| (ip, *port))
        })
        .collect();

    let concurrency = concurrency.max(1);

    let open = stream::iter(targets)
        .map(|(ip, port)| async move {
            let addr = SocketAddr::from((ip, port));
            match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(addr)).await {
                Ok(Ok(_stream)) => Some((ip, port)),
                _ => None,
            }
        })
        .buffer_unordered(concurrency)
        .filter_map(|result| async move { result })
        .collect::<Vec<(Ipv4Addr, u16)>>()
        .await;

    let mut hosts: Vec<ScannedHost> = Vec::new();
    for (ip, port) in open {
        let ip_string = ip.to_string();
        match hosts.iter_mut().find(|host| host.ip == ip_string) {
            Some(host) => host.open_ports.push(port),
            None => hosts.push(ScannedHost {
                ip: ip_string,
                open_ports: vec![port],
            }),
        }
    }

    for host in &mut hosts {
        host.open_ports.sort_unstable();
        host.open_ports.dedup();
    }
    hosts.sort_by_key(|host| {
        host.ip
            .parse::<Ipv4Addr>()
            .map(u32::from)
            .unwrap_or_default()
    });

    hosts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn finds_open_tcp_port() {
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("listener should bind");
        let port = listener.local_addr().expect("local addr").port();

        let server = tokio::spawn(async move { while listener.accept().await.is_ok() {} });

        let hosts = scan_range(
            Ipv4Addr::new(127, 0, 0, 1),
            Ipv4Addr::new(127, 0, 0, 1),
            &[port],
            Duration::from_millis(500),
            4,
        )
        .await;

        server.abort();

        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].ip, "127.0.0.1");
        assert_eq!(hosts[0].open_ports, vec![port]);
    }

    #[tokio::test]
    async fn deduplicates_ports_of_same_host() {
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("listener should bind");
        let port = listener.local_addr().expect("local addr").port();

        let server = tokio::spawn(async move { while listener.accept().await.is_ok() {} });

        let hosts = scan_range(
            Ipv4Addr::new(127, 0, 0, 1),
            Ipv4Addr::new(127, 0, 0, 1),
            &[port, port],
            Duration::from_millis(500),
            4,
        )
        .await;

        server.abort();

        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].open_ports, vec![port]);
    }

    #[tokio::test]
    async fn returns_empty_list_when_nothing_responds() {
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("listener should bind");
        let port = listener.local_addr().expect("local addr").port();
        drop(listener);

        let hosts = scan_range(
            Ipv4Addr::new(127, 0, 0, 1),
            Ipv4Addr::new(127, 0, 0, 1),
            &[port],
            Duration::from_millis(200),
            4,
        )
        .await;

        assert!(hosts.is_empty());
    }
}
