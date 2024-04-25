use std::{net::SocketAddr, ops::DerefMut, time::Duration};

use futures::lock::Mutex;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::tcp::{OwnedReadHalf, OwnedWriteHalf}};

use crate::frame::{NetworkFrame, TNetworkFrame};

use canzero_common::CanFrame;

#[derive(Debug)]
pub struct TcpCan {
    tx_stream : Mutex<OwnedWriteHalf>,
    rx_stream : Mutex<(Vec<u8>,OwnedReadHalf)>,
}

impl TcpCan {
    pub fn new(tcp_stream: tokio::net::TcpStream) -> Self {
        let frame_size = bincode::serialized_size(&TNetworkFrame::new(Duration::from_secs(0), NetworkFrame{
            bus_id : 0,
            can_frame : CanFrame::new(0, false, false, 0, 0),
        })).unwrap();

        let (rx, tx) = tcp_stream.into_split();
        Self {
            tx_stream : Mutex::new(tx),
            rx_stream : Mutex::new((vec![0;frame_size as usize], rx)),
        }
    }

    pub async fn send(&self, frame: &TNetworkFrame) {
        let bytes = bincode::serialize(frame).unwrap();
        match self.tx_stream.lock().await.write_all(&bytes).await {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Failed to send on tcp stream, failed with {err:?}");
            }
        };

    }

    pub async fn recv(&self) -> Option<TNetworkFrame> {
        let mut rx_lock = self.rx_stream.lock().await;
        let (rx_buffer,rx_stream) = rx_lock.deref_mut();
        match rx_stream.read_exact(rx_buffer).await {
            Ok(_) => Some(bincode::deserialize::<TNetworkFrame>(rx_buffer).unwrap()),
            Err(_) => None,
        }
    }
    pub async fn addr(&self) -> SocketAddr {
        self.tx_stream.lock().await.peer_addr().unwrap()
    }
}
