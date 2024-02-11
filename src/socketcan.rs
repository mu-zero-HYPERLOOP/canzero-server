use can_socketcan_platform_rs::{
    frame::{CanError, CanFrame},
    CanSocket,
};

use crate::errors::{Error, Result};

#[derive(Clone)]
pub struct SocketCan {
    socket: CanSocket,
    interface: String,
}

impl SocketCan {
    pub fn create(interface: &str) -> Result<Self> {
        let socket = match CanSocket::open(interface) {
            Ok(owned_socket) => owned_socket,
            Err(err) => {
                return Err(Error::FailedToCreateSocketCanSocket {
                    interface: interface.to_owned(),
                    io_error: err,
                })
            }
        };
        Ok(Self {
            socket,
            interface: interface.to_owned(),
        })
    }
    pub fn transmit(&self, can_frame: CanFrame) -> Result<()> {
        match self.socket.transmit(&can_frame) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::FailedToWriteToSocketCan {
                interface: self.interface.clone(),
                io_error: err,
            }),
        }
    }
    pub fn receive(&self) -> Result<std::result::Result<CanFrame, CanError>> {
        match self.socket.receive() {
            Ok(frame) => Ok(frame),
            Err(err) => Err(Error::FailedToReadFromSocketCan {
                interface: self.interface.clone(),
                io_error: err,
            }),
        }
    }
}
