
use crate::frame::NetworkFrame;



pub struct TcpCan {
    tcp_stream : tokio::net::TcpStream,
}

impl TcpCan {

    pub fn new(tcp_stream : tokio::net::TcpStream) -> Self {
        Self {
            tcp_stream
        }
    }

    pub async fn send(&self, frame: &NetworkFrame) {
        todo!()
    }

    pub async fn recv(&self) -> Option<NetworkFrame> {
        todo!()
    }
}
