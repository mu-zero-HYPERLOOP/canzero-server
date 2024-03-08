use crate::{bus_frame::BusFrame, errors::{Error, Result}, socketcan::SocketCan};

#[derive(Clone)]
pub struct SocketCanMuxSender {
    socketcans: Vec<SocketCan>,
}

impl SocketCanMuxSender {
    pub fn new(socketcans: Vec<SocketCan>) -> Self{
        Self { socketcans }
    }

    pub fn send(&self, bus_frame: BusFrame) -> Result<()> {
        match self.socketcans.get(bus_frame.bus_id as usize) {
            Some(socket) => socket.transmit(bus_frame.can_frame),
            None => Err(Error::InvalidBusId {
                bus_id: bus_frame.bus_id,
            }),
        }
    }

}
