use crate::{frame::NetworkFrame, socketcan::SocketCan, tcpcan::TcpCan};

pub enum NetworkNode {
    SocketCanNode(SocketCan),
    TcpCanNode(TcpCan),
}

impl NetworkNode {
    pub async fn send(&self, frame: &NetworkFrame) {
        match &self {
            NetworkNode::SocketCanNode(socketcan) => socketcan.send(frame).await,
            NetworkNode::TcpCanNode(tcpcan) => tcpcan.send(frame).await,
        }
    }

    pub async fn recv(&self) -> Option<NetworkFrame> {
        match &self {
            NetworkNode::SocketCanNode(socketcan) => socketcan.recv().await,
            NetworkNode::TcpCanNode(tcpcan) => tcpcan.recv().await,
        }
    }
}

