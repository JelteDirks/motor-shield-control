use rppal::gpio::{OutputPin};

pub struct Servo {
    pin: Option<OutputPin>,
    angle: u8,
    config: ServoConfig,
}

pub struct ServoConfig {
    cycle: Duration,
    width: Duration,
    angle: u8
}
