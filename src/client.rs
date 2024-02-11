use std::net::TcpStream;

use crate::{
    errors::{Error, Result},
    socketcan_sender::SocketCanMuxSender,
    tcp_stream::CanTcpStream,
};
use crate::{socketcan::SocketCan, socketcan_receiver::SocketCanSingleReceiver};

pub fn run_client(host: &str, socketcan_interfaces: Vec<SocketCan>) -> Result<()> {
    let Ok(stream) = TcpStream::connect(host) else {
        return Err(Error::FailedToConnectToTcpServer {
            address: host.to_owned(),
        });
    };

    let server_address = match stream.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "(Unknown)".to_owned(),
    };

    println!("TCP-Client connected to {server_address}");

    let stream = CanTcpStream::new(stream);


    let socketcan_single_receiver = SocketCanSingleReceiver::new(socketcan_interfaces.clone());

    let rx_shutdown = socketcan_single_receiver.shutdown_handle();

    // Receive from TCP-Server and forward to tx.
    let mut rx_stream = stream.try_clone()?;
    let socketcan_mux_sender = SocketCanMuxSender::new(socketcan_interfaces);
    let rx_server_address = server_address.clone();
    std::thread::spawn(move || {
        loop {
            match rx_stream.receive() {
                Ok(bus_frame) => {
                    socketcan_mux_sender.send(bus_frame).unwrap_or_else(|err| {
                        eprintln!("{err:?}");
                    });
                }
                Err(_) => {
                    rx_shutdown.shutdown();
                    break;
                }
            }
        }
        println!("Shutdown Connection {rx_server_address} : Rx gracefully shutdown");
    });

    let mut tx_stream = stream.try_clone()?;
    loop {
        match socketcan_single_receiver.receive() {
            Ok(bus_frame) => match tx_stream.send(&bus_frame) {
                Ok(_) => (),
                Err(err) => eprintln!("{err:?}"),
            },
            Err(_) => break,
        }
    }
    println!("Shutdown Connection {server_address} : Tx gracefully shutdown");
    Ok(())
}
