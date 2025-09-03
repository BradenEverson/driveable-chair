//! Serial Connected ODrive Controller

use serialport::SerialPort;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;

/// An ODrive Controller that we can send ASCII commands to
pub struct OdriveController {
    port: Box<dyn SerialPort>,
}

/// Selectable motor types
#[repr(u8)]
pub enum Motor {
    /// Motor 0
    Zero,
    /// Motor 1
    One,
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

    /// Sends and flushes a command to the odrive
    fn send_command(&mut self, cmd: &str) -> std::io::Result<()> {
        self.port.write(cmd.as_bytes())?;
        self.port.flush()
    }

    /// Sends the velocity setting command
    pub fn velocity(
        &mut self,
        motor: Motor,
        velocity: u16,
        torque_ff: Option<u16>,
    ) -> std::io::Result<()> {
        let motor = motor as u8;
        self.send_command(&format!("w axis{motor}.requested_state 8\n"))?;
        self.send_command(&format!("w axis{motor}.controller.config.control_mode 2\n"))?;
        self.send_command(&format!("w axis{motor}.controller.config.input_mode 1\n"))?;

        let cmd = if let Some(torque_ff) = torque_ff {
            format!("v {motor} {velocity} {torque_ff}\n")
        } else {
            format!("v {motor} {velocity}\n")
        };

        println!("Sending {cmd}");

        self.send_command(&cmd)
    }
}
