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

    /// Sends the velocity setting command
    pub fn velocity(
        &mut self,
        motor: u8,
        velocity: u16,
        torque_ff: Option<u16>,
    ) -> std::io::Result<()> {
        let cmd = if let Some(torque_ff) = torque_ff {
            format!("v {motor} {velocity} {torque_ff}")
        } else {
            format!("v {motor} {velocity}")
        };

        self.port.write(cmd.as_bytes())?;
        self.port.flush()
    }
}
