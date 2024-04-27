use std::{net::SocketAddr, sync::Arc};

use can_config_rs::config;

use crate::socketcan::SocketCan;
use crate::tcpcan::TcpCan;

use self::udp_discover::start_udp_discover;

pub mod udp_discover;

pub async fn start_client(config: &config::NetworkRef) {
    loop {
        start_client_once(config).await;
    }
}

pub async fn start_client_once(config: &config::NetworkRef) {
    let server_addr: SocketAddr;
    let server_name : String;
    println!("\u{1b}[33mSearching for server\u{1b}[33m");
    loop {
        let connections = start_udp_discover("CANzero", 9002).await.unwrap();
        if !connections.is_empty() {
            server_addr = SocketAddr::new(connections[0].server_addr, connections[0].service_port);
            server_name = connections[0].server_name.clone();
            break;
        }
    }

    let tcp_stream = tokio::net::TcpStream::connect(server_addr).await.unwrap();
    let tcpcan = Arc::new(TcpCan::new(tcp_stream));

    println!("\u{1b}[32mSuccessful connection to {server_name} at {server_addr}\u{1b}[0m");

    let socketcan = Arc::new(SocketCan::create(config.buses()).unwrap());

    let tcp_rx = tcpcan.clone();
    let socketcan_tx = socketcan.clone();
    let handle = tokio::spawn(async move {
        loop {
            let Some(frame) = socketcan.recv().await else {
                break;
            };
            if let Err(err) = tcpcan.send(&frame).await {
                eprintln!("{err:?}");
            };
        }
    });

    loop {
        let Some(frame) = tcp_rx.recv().await else {
            break;
        };
        if let Err(err) = socketcan_tx.send(&frame).await {
            eprintln!("{err:?}");
        };
    }
    handle.abort();
    println!("\u{1b}[31mConnection closed\u{1b}[0m");
}
