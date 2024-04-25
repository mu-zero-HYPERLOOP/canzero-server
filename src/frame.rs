use serde::{Deserialize, Serialize};

use canzero_common::{CanFrame, Timestamped};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkFrame {
    pub bus_id : u32,
    pub can_frame : CanFrame,
}

pub type TNetworkFrame = Timestamped<NetworkFrame>;

