use std::{net::SocketAddr, time::Duration};

use can_config_rs::config;

use crate::{
    frame::NetworkFrame,
    socketcan::{self, SocketCan},
};

use self::udp_discover::start_udp_discover;

pub mod udp_discover;

pub async fn start_client(config: &config::NetworkRef) {
    let (tx, rx) = tokio::sync::mpsc::channel::<NetworkFrame>(16);

    let config = config.clone();

    let mut server_addr: SocketAddr;
    loop {
        let connections = start_udp_discover("CANzero", 9002).await.unwrap();
        if !connections.is_empty() {
            server_addr = SocketAddr::new(connections[0].0, connections[0].1);
            break;
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    }

    let mut tcp_stream = tokio::net::TcpStream::connect(server_addr).await.unwrap();
    let (read_stream, write_stream) = tcp_stream.split();

    let socketcan = SocketCan::create(&config.buses()).unwrap();
    tokio::spawn(async move {
        loop {
            let Some(frame) = socketcan.recv().await else {
                break;
            };
            // write_stream -> write
        }
    });

    loop {
        // receive from read_stream


        // socketcan.send(frame);
    }
}
