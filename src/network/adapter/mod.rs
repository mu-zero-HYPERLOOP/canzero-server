use self::client::TcpClient;



pub mod client;
pub mod socketcan;

pub enum NetworkAdapter {
    TcpClient(TcpClient),
    SocketCan,
}

