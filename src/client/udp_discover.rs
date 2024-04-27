use std::time::{Duration, Instant};

use crate::frame::{HelloFrame, NetworkDescription, UdpDiscoverFrame};

pub async fn start_udp_discover(
    service_name: &str,
    broadcast_port: u16,
) -> std::io::Result<Vec<NetworkDescription>> {
    let socket = tokio::net::UdpSocket::bind(&format!("0.0.0.0:0"))
        .await
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::AddrInUse,
                "UdpBeacon already hosted!".to_owned(),
            )
        })?;

    socket.set_broadcast(true)?;

    let broadcast_addr = format!("255.255.255.255:{broadcast_port}");

    let hello_frame = bincode::serialize(&UdpDiscoverFrame::Hello(HelloFrame {
        service_name: service_name.to_owned(),
    }))
    .expect("Failed to serialize udp discovery HelloFrame");
    println!("\u{1b}[34mUDP-Discover: Sending hello packet\u{1b}[0m");
    socket.send_to(&hello_frame, broadcast_addr).await?;

    let mut rx_buffer = [0u8; 1024];
    let mut connections = vec![];
    loop {
        match tokio::time::timeout(
            Duration::from_millis(1000),
            socket.recv_from(&mut rx_buffer),
        )
        .await
        {
            Ok(Ok((packet_size, udp_server_addr))) => {
                let local_timebase = Instant::now();
                let Ok(frame) = bincode::deserialize::<UdpDiscoverFrame>(&rx_buffer[..packet_size])
                else {
                    println!("\u{1b}[34mUDP-Discover: Received ill formed frame [ignored]\u{1b}[0m");
                    continue;
                };
                let UdpDiscoverFrame::NDF(ndf) = frame else {
                    continue;
                };
                if ndf.service_name != service_name {
                    println!(
                        "\u{1b}[34mUDP-Discover: Received ndf for service {} [ignored]\u{1b}[0m",
                        ndf.service_name
                    );
                    continue;
                }
                let nd = NetworkDescription {
                    server_addr: udp_server_addr.ip(),
                    server_name: ndf.server_name,
                    service_port: ndf.service_port,
                    timebase: local_timebase - ndf.time_since_sor,
                };
                println!("\u{1b}[34mUDP-Discover: Discovered server named {} at {:?}:{:?}\u{1b}[0m", &nd.server_name, nd.server_addr, nd.service_port);
                connections.push(nd);
                break;
            }
            Err(_) => {
                println!("\u{1b}[34mUDP-Discover: Response timed out\u{1b}[0m");
                break;
            }
            _ => continue,
        }
    }
    Ok(connections)
}
