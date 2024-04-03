use can_socketcan_platform_rs::frame::CanFrame;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkFrame {
    pub bus_id : u32,
    pub can_frame : CanFrame,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TNetworkFrame {
    pub network_frame : NetworkFrame,
    pub timestamp_us : u128,
}

