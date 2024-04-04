use std::sync::Arc;

const MAX_REFLECTOR_FRAME_SIZE: usize = 1024;

pub async fn start_udp_reflector(
    service_name: &str,
    service_port: u16,
    listening_port: u16,
) -> std::io::Result<()> {
    let socket = Arc::new(tokio::net::UdpSocket::bind(&format!("0.0.0.0:{listening_port}"))
        .await
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::AddrInUse,
                "UdpBeacon already hosted!".to_owned(),
            )
        })?);

    socket.set_broadcast(true)?;

    println!(
        "\u{1b}[34mUDP-Reflector: bind socket at {}\u{1b}[0m",
        socket.local_addr()?.port()
    );

    let service_name = service_name.to_owned();

    tokio::spawn(async move {
        loop {
            loop {
                let mut rx_buffer = [0; MAX_REFLECTOR_FRAME_SIZE];
                println!("\u{1b}[34mUDP-Reflector: listening\u{1b}[0m");
                let (number_of_bytes, source_addr) = socket
                    .recv_from(&mut rx_buffer)
                    .await
                    .expect("Failed to receive from UDP socket");
                println!("\u{1b}[34mUDP-Reflector: received hello from {source_addr}\u{1b}[0m");
                let service_name = service_name.clone();
                let socket = socket.clone();
                tokio::spawn(async move {
                    let ty = rx_buffer[0];
                    let source_service_name =
                        std::str::from_utf8(&rx_buffer[1..number_of_bytes]).unwrap();
                    if ty == 0 && source_service_name == service_name {
                        // respond!
                        let mut tx_buffer = vec![1u8];
                        tx_buffer.extend_from_slice(unsafe {
                            &std::mem::transmute::<u16, [u8; 2]>(service_port)
                        });
                        tx_buffer.extend_from_slice(&service_name.as_bytes());
                        println!("\u{1b}[34mUDP-Reflector: responding to {source_addr}\u{1b}[33m");
                        let Ok(_) = socket.send_to(&tx_buffer, &source_addr).await else {
                            println!("\u{1b}[34mUDP-Reflector: Failed to respond {source_addr}\u{1b}[33m"); return;
                        };
                    }
                });
            }
        }
    });

    Ok(())
}
