#[cfg(feature = "socket-can")]
use crate::socketcan::SocketCan;

use crate::{frame::TNetworkFrame, tcpcan::TcpCan};

#[derive(Debug)]
pub enum NetworkNode {
    #[cfg(feature = "socket-can")]
    SocketCanNode(SocketCan),
    TcpCanNode(TcpCan),
}

impl NetworkNode {
    pub async fn send(&self, frame: &TNetworkFrame) -> std::io::Result<()> {
        match &self {
            #[cfg(feature = "socket-can")]
            NetworkNode::SocketCanNode(socketcan) => socketcan.send(frame).await,
            NetworkNode::TcpCanNode(tcpcan) => tcpcan.send(frame).await,
        }
    }

    pub async fn recv(&self) -> Option<TNetworkFrame> {
        match &self {
            #[cfg(feature = "socket-can")]
            NetworkNode::SocketCanNode(socketcan) => socketcan.recv().await,
            NetworkNode::TcpCanNode(tcpcan) => tcpcan.recv().await,
        }
    }
}
