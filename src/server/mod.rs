use can_config_rs::config;

use crate::{socketcan::SocketCan, tcpcan::TcpCan};

use self::{
    network::{node::NetworkNode, Network},
    udp_reflector::start_udp_reflector,
};

pub mod network;
pub mod udp_reflector;

pub async fn start_server(config: &config::NetworkRef) {

    let network = Network::new();

    println!("\u{1b}[33mStarting server\u{1b}[0m");

    network.start(NetworkNode::SocketCanNode(
        SocketCan::create(config.buses()).unwrap(),
    )).await;

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();

    let tcp_welcome_port = tcp_listener.local_addr().unwrap().port();

    println!("\u{1b}[33mBind TCP Welcome Socket at {tcp_welcome_port}\u{1b}[0m");

    start_udp_reflector("CANzero", tcp_welcome_port, 9002).await.unwrap();

    println!("\u{1b}[33mSuccessfully started server\u{1b}[0m");

    loop {
        let (stream, addr) = tcp_listener.accept().await.unwrap();
        println!("\u{1b}[32mConnection from {addr:?}\u{1b}[0m");
        network.start(NetworkNode::TcpCanNode(TcpCan::new(stream))).await;
    }

}
