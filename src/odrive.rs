//! Serial Connected ODrive Controller

use serialport::SerialPort;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;

/// An ODrive Controller that we can send ASCII commands to
pub struct OdriveController {
    port: Box<dyn SerialPort>,
}

impl OdriveController {
    /// Create a new ODrive controller from Serial
    pub fn from_port(port: &str) -> serialport::Result<Self> {
        let port = serialport::new(port, 115200)
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_millis(1000))
            .open()?;

        Ok(Self { port })
    }
}
