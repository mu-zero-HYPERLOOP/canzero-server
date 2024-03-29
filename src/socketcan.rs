use std::time::Instant;

use can_config_rs::config;
use can_socketcan_platform_rs::CanSocket;
use tokio::sync::mpsc;

use crate::frame::{NetworkFrame, TNetworkFrame};

pub struct SocketCan {
    sockets: Vec<(CanSocket, String)>,
    rx: tokio::sync::Mutex<mpsc::Receiver<TNetworkFrame>>,
}

impl SocketCan {

    pub fn create(buses: &Vec<config::bus::BusRef>) -> std::io::Result<Self> {
        let sockets: Vec<(CanSocket, String)> = buses
            .iter()
            .map(|bus| (CanSocket::open(bus.name()).unwrap(), bus.name().to_owned()))
            .collect();

        let (tx, rx) = mpsc::channel(16);

        let rx_sockets = sockets.clone();

        let start_of_run = Instant::now();

        for (bus_id, (socket, ifname)) in rx_sockets.into_iter().enumerate() {
            let tx = tx.clone();
            tokio::task::spawn_blocking(move || loop {
                match socket.receive() {
                    Ok(frame) => {
                        let tx = tx.clone();
                        let ifname = ifname.clone();
                        tokio::spawn(async move {
                            let frame = match frame {
                                Ok(can_frame) => NetworkFrame {
                                    bus_id: bus_id as u32,
                                    can_frame,
                                },
                                Err(_) => todo!(),
                            };
                            let tframe = TNetworkFrame{
                                network_frame : frame,
                                timestamp_us : Instant::now().duration_since(start_of_run).as_micros(),
                            };

                            if let Err(_) = tx.send(tframe).await {
                                eprintln!("Failed to send on SocketCan interface {ifname:?}");
                            }
                        });
                    }
                    Err(_) => {
                        eprintln!("Failed to receive from SocketCAN interface {ifname:?}")
                    }
                }
            });
        }
        Ok(Self {
            sockets,
            rx: tokio::sync::Mutex::new(rx),
        })
    }
    pub async fn send(&self, frame: &TNetworkFrame) {
        if let Err(_) = self.sockets[frame.network_frame.bus_id as usize]
            .0
            .transmit(&frame.network_frame.can_frame)
        {
            eprintln!(
                "Failed to transmit frame on socketcan interface {:?}",
                self.sockets[frame.network_frame.bus_id as usize].1,
            );
        };
    }
    pub async fn recv(&self) -> Option<TNetworkFrame> {
        self.rx.lock().await.recv().await
    }
}

impl std::fmt::Debug for SocketCan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SocketCAN")
    }
}
