use can_socketcan_platform_rs::frame::CanFrame;

pub struct BusFrame {
    pub can_frame : CanFrame,
    pub bus_id : u32,
}
