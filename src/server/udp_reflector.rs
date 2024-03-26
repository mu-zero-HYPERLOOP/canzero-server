
const MAX_REFLECTOR_FRAME_SIZE: usize = 1024;

pub async fn start_udp_reflector(
    service_name: &str,
    service_port: u16,
    listening_port: u16,
) -> std::io::Result<()> {
    let socket = tokio::net::UdpSocket::bind(&format!("0.0.0.0:{listening_port}"))
        .await
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::AddrInUse,
                "UdpBeacon already hosted!".to_owned(),
            )
        })?;


    socket.set_broadcast(true)?;

    println!("UDP-Reflector: bind socket at {}", socket.local_addr()?);

    let service_name = service_name.to_owned();

    tokio::spawn(async move {
        loop {
            let mut rx_buffer = [0; MAX_REFLECTOR_FRAME_SIZE];
            loop {
                println!("UDP-Reflector: listening");
                let (number_of_bytes, source_addr) = socket
                    .recv_from(&mut rx_buffer)
                    .await
                    .expect("Failed to receive from UDP socket");
                println!("UDP-Reflector: received hello from {source_addr}");
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
                    println!("UDP-Reflector: responding to {source_addr}");
                    socket.send_to(&tx_buffer, &source_addr).await.unwrap();
                }
            }
        }
    });

    Ok(())
}
