use can_config_rs::config;

use crate::{socketcan::SocketCan, tcpcan::TcpCan};

use self::{
    network::{node::NetworkNode, Network},
    udp_reflector::start_udp_reflector,
};

pub mod network;
pub mod udp_reflector;

pub async fn start_server(config: &config::NetworkRef) {
    // create TCP welcome socket!

    let network = Network::new();

    println!("start server");

    network.start(NetworkNode::SocketCanNode(
        SocketCan::create(config.buses()).unwrap(),
    )).await;

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();

    let tcp_welcome_port = tcp_listener.local_addr().unwrap().port();

    println!("Bind TCP Welcome Socket at {tcp_welcome_port}");

    start_udp_reflector("CANzero", tcp_welcome_port, 9002).await.unwrap();

    loop {
        let (stream, addr) = tcp_listener.accept().await.unwrap();
        println!("Connection from {addr:?}");
        network.start(NetworkNode::TcpCanNode(TcpCan::new(stream))).await;
    }

}
