use std::sync::Arc;

use crossbeam::channel::{self, Receiver};
use crossbeam::{channel::Sender, sync::ShardedLock};

use crate::errors::{Error, Result};
use crate::{bus_frame::BusFrame, socketcan::SocketCan};

pub enum Msg {
    Frame(BusFrame),
    Poison,
}

pub struct SocketCanMultiReceiver {
    tcp_client_endpoints: Arc<ShardedLock<Vec<Sender<Msg>>>>,
}

impl SocketCanMultiReceiver {
    pub fn new(socketcan_interfaces: Vec<SocketCan>) -> Self {
        // ========= Create communication channels for SocketCAN to TCP-Clients ===========
        let tcp_client_dispatch_endpoints: Arc<ShardedLock<Vec<Sender<Msg>>>> =
            Arc::new(ShardedLock::new(vec![]));
        // NOTE: multiple producers (fixed) to multiple consumers (dyn)
        // 1. Start thread for each socketcan interface
        // 2. When receiving a frame forward them to the tcp_client_dispatch_endpoints
        for (bus_id, socketcan_ref) in socketcan_interfaces.into_iter().enumerate() {
            let tcp_client_dispatch_endpoints = tcp_client_dispatch_endpoints.clone();
            std::thread::spawn(move || {
                let bus_id = bus_id as u32;
                let mut closed_endpoint_indicies: Vec<usize> = vec![];
                loop {
                    closed_endpoint_indicies.clear();
                    match socketcan_ref.receive() {
                        Ok(frame) => match tcp_client_dispatch_endpoints.read() {
                            Ok(tcp_client_endpoints) => {
                                for i in 0..tcp_client_endpoints.len() {
                                    match frame.clone() {
                                        Ok(can_frame) => {
                                            match tcp_client_endpoints[i as usize]
                                                .send(Msg::Frame(BusFrame { can_frame, bus_id }))
                                            {
                                                Ok(_) => (),
                                                Err(err) => {
                                                    closed_endpoint_indicies.push(i);
                                                }
                                            }
                                        }
                                        Err(can_error) => todo!(),
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("Failed to acquire tcp_client_receivers lock: {err:?}")
                            }
                        },
                        Err(err) => {
                            eprintln!("Failed to receive from socketcan interface: {err:?}")
                        }
                    }
                    for closed_endpoint in closed_endpoint_indicies.iter().rev() {
                        match tcp_client_dispatch_endpoints.write() {
                            Ok(mut tcp_client_endpoints) => {
                                tcp_client_endpoints.remove(*closed_endpoint);
                            }
                            Err(err) => eprintln!("{err:?}"),
                        }
                    }
                }
            });
        }
        return Self {
            tcp_client_endpoints: tcp_client_dispatch_endpoints,
        };
    }

    pub fn add_rx(&self) -> Result<(SocketCanRx, SocketCanRxShutdownHandle)> {
        match self.tcp_client_endpoints.write() {
            Ok(mut tcp_client_endpoints) => {
                let (tcp_tx, tcp_rx) = crossbeam::channel::bounded(100);
                tcp_client_endpoints.push(tcp_tx.clone());
                return Ok((
                    SocketCanRx { rx: tcp_rx },
                    SocketCanRxShutdownHandle { tx: tcp_tx },
                ));
            }
            Err(_) => {
                return Err(Error::FailedToAcquireLock {
                    name: "Failed to acquire read lock on tcp_client endpoint".to_owned(),
                })
            }
        }
    }
}

pub struct SocketCanRx {
    rx: Receiver<Msg>,
}

impl SocketCanRx {
    pub fn recv(&self) -> Result<BusFrame> {
        match self.rx.recv() {
            Ok(msg) => match msg {
                Msg::Frame(bus_frame) => Ok(bus_frame),
                Msg::Poison => Err(Error::ChannelPoison),
            },
            Err(_) => Err(Error::CrossbeamChannelClosed),
        }
    }
}

pub struct SocketCanRxShutdownHandle {
    tx: Sender<Msg>,
}
impl SocketCanRxShutdownHandle {
    pub fn shutdown(&self) {
        &self.tx.send(Msg::Poison);
    }
}

pub struct SocketCanSingleReceiver {
    tx: channel::Sender<Msg>,
    rx: channel::Receiver<Msg>,
}

impl SocketCanSingleReceiver {
    pub fn new(socketcan_interfaces: Vec<SocketCan>) -> Self {
        let (tx, rx) = channel::bounded::<Msg>(128);

        for (bus_id, socketcan_interface) in socketcan_interfaces.into_iter().enumerate() {
            let bus_id = bus_id as u32;
            let tx = tx.clone();
            std::thread::spawn(move || loop {
                match socketcan_interface.receive() {
                    Ok(frame) => match frame {
                        Ok(can_frame) => {
                            tx.send(Msg::Frame(BusFrame { can_frame, bus_id }))
                                .unwrap_or_else(|err| {
                                    eprintln!("{err:?}");
                                });
                        }
                        Err(can_error) => todo!("handle CAN-Error frames"),
                    },
                    Err(err) => eprintln!("{err:?}"),
                }
            });
        }

        Self { tx, rx }
    }

    pub fn receive(&self) -> Result<BusFrame> {
        match self.rx.recv() {
            Ok(msg) => match msg {
                Msg::Frame(bus_frame) => Ok(bus_frame),
                Msg::Poison => Err(Error::ChannelPoison),
            },
            Err(_) => Err(Error::CrossbeamChannelClosed),
        }
    }

    pub fn shutdown_handle(&self) -> SocketCanRxShutdownHandle {
        SocketCanRxShutdownHandle {
            tx: self.tx.clone(),
        }
    }
}
