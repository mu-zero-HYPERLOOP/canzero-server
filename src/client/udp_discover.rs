use std::{time::Duration, net::IpAddr};

pub async fn start_udp_discover(service_name: &str, broadcast_port: u16) -> std::io::Result<Vec<(IpAddr, u16)>> {
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

    let mut hello_msg = vec![0u8];
    hello_msg.extend_from_slice(service_name.as_bytes());
    socket.send_to(&hello_msg, broadcast_addr).await?;

    let mut rx_buffer = [0u8; 1024];
    let mut connections = vec![];
    loop {
        match tokio::time::timeout(Duration::from_millis(100), socket.recv_from(&mut rx_buffer))
            .await
        {
            Ok(Ok((packet_size, server_addr))) => {
                let ty = rx_buffer[0];
                let server_port = (rx_buffer[1] as u16) | ((rx_buffer[2] as u16) << 8);
                let server_service_name = std::str::from_utf8(&rx_buffer[3..packet_size]).unwrap();
                if ty == 1u8 && server_service_name == service_name {
                    connections.push((server_addr.ip(), server_port));
                }
            },
            Err(_) => break,
            _ => continue,
        }
    }
    Ok(connections)
}
