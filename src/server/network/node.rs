use crate::{frame::TNetworkFrame, socketcan::SocketCan, tcpcan::TcpCan};

#[derive(Debug)]
pub enum NetworkNode {
    SocketCanNode(SocketCan),
    TcpCanNode(TcpCan),
}

impl NetworkNode {
    pub async fn send(&self, frame: &TNetworkFrame) {
        match &self {
            NetworkNode::SocketCanNode(socketcan) => socketcan.send(frame).await,
            NetworkNode::TcpCanNode(tcpcan) => tcpcan.send(frame).await,
        }
    }

    pub async fn recv(&self) -> Option<TNetworkFrame> {
        match &self {
            NetworkNode::SocketCanNode(socketcan) => socketcan.recv().await,
            NetworkNode::TcpCanNode(tcpcan) => tcpcan.recv().await,
        }
    }
}

