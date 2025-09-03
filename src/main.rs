//! Main Chair Runtime

use driveable_chair::odrive::OdriveController;

fn main() {
    for port in serialport::available_ports().expect("No serial ports") {
        println!("{port:?}");
    }

    let odrive = OdriveController::from_port("foo").expect("No odrive :(");
}
