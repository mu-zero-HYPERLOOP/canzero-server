use std::{mem::size_of, net::SocketAddr};

use futures::lock::Mutex;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::tcp::{OwnedReadHalf, OwnedWriteHalf}};

use crate::frame::TNetworkFrame;

#[derive(Debug)]
pub struct TcpCan {
    tx_stream : Mutex<OwnedWriteHalf>,
    rx_stream : Mutex<OwnedReadHalf>,
}

impl TcpCan {
    pub fn new(tcp_stream: tokio::net::TcpStream) -> Self {
        let (rx, tx) = tcp_stream.into_split();
        Self {
            tx_stream : Mutex::new(tx),
            rx_stream : Mutex::new(rx),
        }
    }

    pub async fn send(&self, frame: &TNetworkFrame) {
        println!("TcpCan sending {frame:?}");
        let byte_slice: &[u8] = unsafe {
            ::core::slice::from_raw_parts(
                (frame as *const TNetworkFrame) as *const u8,
                size_of::<TNetworkFrame>(),
            )
        };
        match self.tx_stream.lock().await.write_all(byte_slice).await {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Failed to send on tcp stream, failed with {err:?}");
            }
        };

    }

    pub async fn recv(&self) -> Option<TNetworkFrame> {
        let mut buffer: [u8; size_of::<TNetworkFrame>()] = [0; size_of::<TNetworkFrame>()];
        let x = match self.rx_stream.lock().await.read_exact(&mut buffer).await {
            Ok(_) => Some(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
            Err(_) => None,
        };
        println!("TcpCan recv {x:?}");
        x
    }
    pub async fn addr(&self) -> SocketAddr {
        self.tx_stream.lock().await.peer_addr().unwrap()
    }
}
