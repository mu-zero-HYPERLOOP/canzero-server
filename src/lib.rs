

pub mod server;
pub mod frame;
pub mod tcpcan;
pub mod discovery;

// client only allowed avaiable if socket-can feature!
#[cfg(feature = "socket-can")]
pub mod socketcan;
#[cfg(feature = "socket-can")]
pub mod client;
