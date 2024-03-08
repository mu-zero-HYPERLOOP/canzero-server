use std::net::TcpListener;

use crate::errors::Error;
use crate::socketcan_receiver::{SocketCanRx, SocketCanRxShutdownHandle};
use crate::{errors::Result, socketcan_receiver::SocketCanMultiReceiver, tcp_stream::CanTcpStream};
use crate::{socketcan::SocketCan, socketcan_sender::SocketCanMuxSender};

pub fn run_server(address: &str, socketcan_interfaces: Vec<SocketCan>) -> Result<()> {
    let Ok(listener) = TcpListener::bind(address) else {
        return Err(Error::FailedToCreateTcpListener {
            address: address.to_owned(),
        });
    };

    let socketcan_multi_receiver = SocketCanMultiReceiver::new(socketcan_interfaces.clone());
    let socketcan_mux_sender = SocketCanMuxSender::new(socketcan_interfaces);

    println!("TCP-Server started: listening for connections on {address}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_address = match stream.peer_addr() {
                    Ok(addr) => addr.to_string(),
                    Err(_) => "(Unknown)".to_owned(),
                };
                println!("Connection from {stream_address}");

                let (rx, rx_shutdown) = match socketcan_multi_receiver.add_rx() {
                    Ok(rx) => rx,
                    Err(err) => {
                        eprintln!("{err:?}");
                        continue;
                    }
                };
                handle_client(
                    CanTcpStream::new(stream),
                    socketcan_mux_sender.clone(),
                    rx,
                    rx_shutdown,
                    stream_address,
                );
            }
            Err(err) => eprintln!("Failed to connect to incomming connection : {err:?}"),
        }
    }

    Ok(())
}

fn handle_client(
    stream: CanTcpStream,
    tx: SocketCanMuxSender,
    rx: SocketCanRx,
    rx_shutdown: SocketCanRxShutdownHandle,
    client_address: String,
) {
    // Receive from rx and forward to TCP-Client.
    let mut tx_stream = stream.try_clone().expect("Failed to clone TcpStream (tx)");
    let tx_client_address = client_address.clone();
    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(bus_frame) => {
                    tx_stream.send(&bus_frame).unwrap_or_else(|_| { }); //NOTE ignore failed writes
                }
                Err(_) => break,
            };
        }
        println!("Shutdown Connection {tx_client_address} : Tx gracefully shutdown");
    });

    // Receive from TCP-Client and forward to tx.
    let mut rx_stream = stream.try_clone().expect("Failed to clone TcpStream (rx)");
    std::thread::spawn(move || {
        loop {
            match rx_stream.receive() {
                Ok(bus_frame) => match tx.send(bus_frame) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{err:?}"),
                },
                Err(Error::TcpConnectionClosed) => {
                    // TODO graceful shutdown of connection!
                    rx_shutdown.shutdown();
                    break;
                }
                Err(_) => unreachable!(),
            };
        }
        println!("Shutdown Connection {client_address} : Rx gracefully shutdown");
    });
}
