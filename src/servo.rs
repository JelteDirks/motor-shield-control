use rppal::gpio::{OutputPin, Gpio};
use std::time::Duration;

pub struct Servo {
    pin: Option<OutputPin>,
    config: ServoConfig,
}

impl Servo {
    pub fn new(pin: u8) -> Servo  {
        let gpio = match Gpio::new() {
            Ok(g) => g,
            Err(e) => panic!("{:?}", e),
        };

        let gpio_pin = match gpio.get(pin) {
            Ok(gp) => gp,
            Err(e) => panic!("{:?}", e),
        };

        const cycle: Duration = Duration::from_millis(20);
        const width: Duration = Duration::from_micros(1500);

        let mut output_pin = gpio_pin.into_output_low();
        output_pin.set_pwm(cycle, width);

        return Servo {
            pin: Some(output_pin),
            config: ServoConfig {
                cycle: Duration::from_millis(20),
                width: Duration::from_micros(1500),
                angle: 90,
            }
        }
    }
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

    pub fn new_angle(a: u16) -> ServoConfig {
        const maxAngle: u16 = 180;
        const min: f32 = 1000.0;
        const max: f32 = 2000.0;

        if a > maxAngle {
            panic!("angle can be at most 180");
        }

        let range: f32 = max - min;
        let prcnt: f32 = (a as f32) / (maxAngle as f32); 
        let scalar: f32 = prcnt * (range as f32);
        let width: u64 = (min as u64) + (scalar as u64);

        return ServoConfig::new(Duration::from_micros(width));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_angle_calc() {
        let cfg = ServoConfig::new_angle(180);
        assert_eq!(cfg.width, Duration::from_micros(2000)); 
    }

    #[test]
    fn test_90_angle_calc() {
        let cfg = ServoConfig::new_angle(90);
        assert_eq!(cfg.width, Duration::from_micros(1500)); 
    }

    #[test]
    fn test_min_angle_calc() {
        let cfg = ServoConfig::new_angle(0);
        assert_eq!(cfg.width, Duration::from_micros(1000));
    }

    #[test]
    fn test_arbirary_angles() {
        let cfg = ServoConfig::new_angle(52);
        assert_eq!(cfg.width, Duration::from_micros(1288));
    }

    #[test]
    fn test_arbirary_angles2() {
        let cfg = ServoConfig::new_angle(134);
        assert_eq!(cfg.width, Duration::from_micros(1744));
    }
}
