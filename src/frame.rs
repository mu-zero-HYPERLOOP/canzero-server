use std::{net::IpAddr, time::{Duration, Instant}};

use serde::{Deserialize, Serialize};

use canzero_common::{CanFrame, Timestamped};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkFrame {
    pub bus_id : u32,
    pub can_frame : CanFrame,
}

pub type TNetworkFrame = Timestamped<NetworkFrame>;




#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UdpDiscoverFrame {
    Hello(HelloFrame),
    NDF(NetworkDescriptionFrame)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HelloFrame {
    pub service_name : String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkDescriptionFrame {
    pub service_name : String,
    pub service_port : u16,
    pub time_since_sor : Duration,
    pub server_name: String,
}

#[derive(Clone, Debug)]
pub struct NetworkDescription {
    pub timebase : Instant,
    pub server_name: String,
    pub service_port : u16,
    pub server_addr : IpAddr,
}
