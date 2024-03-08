use std::io::{Read, Write};
use std::mem::size_of;
use std::net::TcpStream;

use crate::bus_frame::BusFrame;
use crate::errors::{Error, Result};

pub struct CanTcpStream {
    tcp_stream: TcpStream,
}

impl CanTcpStream {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }

    pub fn send(&mut self, bus_frame: &BusFrame) -> Result<()> {
        let byte_slice: &[u8] = unsafe {
            ::core::slice::from_raw_parts(
                (bus_frame as *const BusFrame) as *const u8,
                size_of::<BusFrame>(),
            )
        };
        match self.tcp_stream.write_all(byte_slice) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::FailedToWriteToTcpStream),
        }
    }
    pub fn receive(&mut self) -> Result<BusFrame> {
        let mut buffer: [u8; size_of::<BusFrame>()] = [0; size_of::<BusFrame>()];
        match self.tcp_stream.read_exact(&mut buffer) {
            Ok(_) => (),
            Err(_) => return Err(Error::TcpConnectionClosed),
        }
        Ok(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
    }

    pub fn try_clone(&self) -> Result<Self> {
        match self.tcp_stream.try_clone() {
            Ok(tcp_stream) => Ok(Self {tcp_stream}),
            Err(_) => Err(Error::FailedToCloneTcpStream),
        }
    }
}

