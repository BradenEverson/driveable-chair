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

    fn send_command(&mut self, cmd: &str) -> std::io::Result<String> {
        self.port.write(cmd.as_bytes())?;
        self.port.flush()?;

        let mut response = String::new();
        let mut buffer = [0u8; 1024];

        loop {
            match self.port.read(&mut buffer) {
                Ok(bytes_read) => {
                    let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
                    response.push_str(&chunk);

                    if chunk.contains('\n') {
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(response.trim().to_string())
    }

    /// Sends the velocity setting command
    pub fn velocity(
        &mut self,
        motor: u8,
        velocity: u16,
        torque_ff: Option<u16>,
    ) -> std::io::Result<()> {
        println!(
            "{}",
            self.send_command(&format!("w axis{motor}.controller.config.control_mode 2"))?
        );
        println!(
            "{}",
            self.send_command(&format!("w axis{motor}.controller.config.input_mode 1"))?
        );

        let cmd = if let Some(torque_ff) = torque_ff {
            format!("v {motor} {velocity} {torque_ff}\r\n")
        } else {
            format!("v {motor} {velocity}\r\n")
        };

        println!("Sending {cmd}");

        println!("{}", self.send_command(&cmd)?);
        Ok(())
    }
}
