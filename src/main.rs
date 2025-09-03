//! Main Chair Runtime

use std::{thread, time::Duration};

use driveable_chair::odrive::{Motor, OdriveController};

/// Serial port of the right odrive wheel
const RIGHT_WHEEL: &'static str = "/dev/ttyACM0";
const LEFT_WHEEL: &'static str = "/dev/ttyACM1";

fn main() {
    let mut right_odrive = OdriveController::from_port(RIGHT_WHEEL).expect("No odrive :(");
    let mut left_odrive = OdriveController::from_port(LEFT_WHEEL).expect("No odrive :(");

    right_odrive
        .velocity(Motor::Zero, 5, None)
        .expect("Failed to set velocity");

    left_odrive
        .velocity(Motor::Zero, -5, None)
        .expect("Failed to set velocity");

    thread::sleep(Duration::from_secs(3));

    right_odrive
        .stop(Motor::Zero)
        .expect("Stop, that's no good");

    left_odrive.stop(Motor::Zero).expect("Stop, that's no good");
}
