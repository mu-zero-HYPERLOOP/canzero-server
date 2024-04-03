use std::{
    net::SocketAddr,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use can_config_rs::{
    builder::bus::BusData,
    config::{
        self,
        bus::{Bus, BusRef},
    },
};

use crate::{socketcan::SocketCan, tcpcan::TcpCan};

use self::udp_discover::start_udp_discover;

pub mod udp_discover;

pub async fn start_client(config: &config::NetworkRef) {
    loop {
        start_client_once(config).await;
    }
}

pub async fn start_client_once(config: &config::NetworkRef) {
    let server_addr: SocketAddr;
    println!("\u{1b}[33mSearching for server\u{1b}[33m");
    loop {
        let connections = start_udp_discover("CANzero", 9002).await.unwrap();
        if !connections.is_empty() {
            server_addr = SocketAddr::new(connections[0].0, connections[0].1);
            break;
        }
    }

    let tcp_stream = tokio::net::TcpStream::connect(server_addr).await.unwrap();
    let tcpcan = Arc::new(TcpCan::new(tcp_stream));

    println!("\u{1b}[32mSuccessful connection to {server_addr}\u{1b}[0m");

    let buses : Vec<BusRef> = config
        .buses()
        .iter()
        .map(|bus| {
            BusRef::new(Bus::new(
                &format!("v{}", bus.name()),
                bus.id(),
                bus.baudrate(),
            ))
        })
        .collect();

    let socketcan = Arc::new(SocketCan::create(&buses).unwrap());

    let tcp_rx = tcpcan.clone();
    let socketcan_tx = socketcan.clone();
    let handle = tokio::spawn(async move {
        loop {
            let Some(frame) = socketcan.recv().await else {
                break;
            };
            tcpcan.send(&frame).await;
        }
    });

    loop {
        let Some(frame) = tcp_rx.recv().await else {
            break;
        };
        socketcan_tx.send(&frame).await;
    }
    handle.abort();
    println!("\u{1b}[31mConnection closed\u{1b}[0m");
}
