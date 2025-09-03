//! Main Chair Runtime

use std::{thread, time::Duration};

use driveable_chair::odrive::OdriveController;

/// Serial port of the right odrive wheel
const RIGHT_WHEEL: &'static str = "/dev/ttyACM0";

fn main() {
    let mut right_odrive = OdriveController::from_port(RIGHT_WHEEL).expect("No odrive :(");

    right_odrive
        .velocity(0, 10, None)
        .expect("Failed to set velocity");

    thread::sleep(Duration::from_secs(5));

    right_odrive
        .velocity(0, 0, None)
        .expect("Failed to set velocity");
}
