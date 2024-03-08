use can_socketcan_platform_rs::frame::CanError;



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidBusId{bus_id : u32},
    FailedToCreateSocketCanSocket{interface : String, io_error : std::io::Error},
    FailedToWriteToSocketCan{interface : String, io_error : std::io::Error},
    SocketCanDisconnected{interface : String, socketcan_error : CanError},
    FailedToReadFromSocketCan{interface : String, io_error : std::io::Error},
    TcpConnectionClosed,
    FailedToWriteToTcpStream,
    FailedToAcquireLock{name : String},
    FailedToCloneTcpStream,
    FailedToConnectToTcpServer{address : String},
    FailedToCreateTcpListener {address : String},
    CrossbeamChannelClosed,
    ChannelPoison,

}
