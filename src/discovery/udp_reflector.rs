use std::{sync::Arc, time::Instant};

use crate::frame::{NetworkDescriptionFrame, UdpDiscoverFrame};

const MAX_REFLECTOR_FRAME_SIZE: usize = 1024;

pub async fn start_udp_reflector(
    service_name: &str,
    service_port: u16,
    listening_port: u16,
    timebase: Instant,
    server_name: &str,
) -> std::io::Result<()> {
    let socket = Arc::new(
        tokio::net::UdpSocket::bind(&format!("0.0.0.0:{listening_port}"))
            .await
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::AddrInUse,
                    "UdpBeacon already hosted!".to_owned(),
                )
            })?,
    );

    socket.set_broadcast(true)?;

    println!(
        "\u{1b}[34mUDP-Reflector: bind socket at {}\u{1b}[0m",
        socket.local_addr()?.port()
    );

    let service_name = service_name.to_owned();
    let server_name = server_name.to_owned();

    tokio::spawn(async move {
        loop {
            loop {
                let mut rx_buffer = [0; MAX_REFLECTOR_FRAME_SIZE];
                println!("\u{1b}[34mUDP-Reflector: listening\u{1b}[0m");
                let (number_of_bytes, source_addr) = socket
                    .recv_from(&mut rx_buffer)
                    .await
                    .expect("Failed to receive from UDP socket");
                let time_since_sor = Instant::now() - timebase;
                println!("\u{1b}[34mUDP-Reflector: received hello from {source_addr}\u{1b}[0m");
                let service_name = service_name.clone();
                let server_name = server_name.clone();
                let socket = socket.clone();
                tokio::spawn(async move {
                    let Ok(frame) =
                        bincode::deserialize::<UdpDiscoverFrame>(&rx_buffer[0..number_of_bytes])
                    else {
                        println!(
                            "\u{1b}[34mUDP-Discover: Received ill formed frame [ignored]\u{1b}[0m"
                        );
                        return;
                    };
                    let UdpDiscoverFrame::Hello(hello_frame) = frame else {
                        return;
                    };
                    if hello_frame.service_name != service_name {
                        println!("\u{1b}[34mUDP-Discover: Received hello from service {} [ignored]\u{1b}[0m",
                        hello_frame.service_name);
                    }
                    let ndf = NetworkDescriptionFrame {
                        service_name,
                        service_port,
                        time_since_sor,
                        server_name,
                    };
                    println!("\u{1b}[34mUDP-Reflector: responding to {source_addr}\u{1b}[33m");
                    let ndf = bincode::serialize(&UdpDiscoverFrame::NDF(ndf)).expect("Failed to serialize NDF frame");
                    let Ok(_) = socket.send_to(&ndf, &source_addr).await else {
                        println!(
                            "\u{1b}[34mUDP-Reflector: Failed to respond {source_addr}\u{1b}[33m"
                        );
                        return;
                    };
                });
            }
        }
    });

    Ok(())
}
