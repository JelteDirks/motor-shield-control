use rppal::gpio::{OutputPin};
use std::time::Duration;

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

impl ServoConfig {

    fn new(width: Duration) -> ServoConfig {
        if width < Duration::from_micros(900) {
            panic!("pulse width should be at least 0.9ms");
        }
 
        if width > Duration::from_micros(2100) {
            panic!("pulse width should be at most 2.1ms");
        }

        return ServoConfig {
            cycle: Duration::from_millis(20),
            width: width,
            angle: 0
        }
    }

    pub fn new_pwm(w: Duration) -> ServoConfig {
        return ServoConfig::new(w);
    }

    pub fn new_angle(a: u8) -> ServoConfig {
        let maxAngle: u16 = 180;
        let min: u16 = 1000;
        let max: u16 = 2000;

        if (a as u16) > maxAngle {
            panic!("angle can be at most 180");
        }

        let range: u16 = max - min;
        let prcnt: u16 = (a as u16) / maxAngle;
        let scalar: u16 = prcnt * range;
        let width: u64 = (min + scalar).into();

        return ServoConfig::new(Duration::from_micros(width));
    }
}
