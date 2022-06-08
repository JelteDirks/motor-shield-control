use rppal::gpio::{OutputPin, Gpio};
use std::time::Duration;
use std::thread::sleep;

pub struct Servo {
    pin: Option<OutputPin>,
    config: ServoConfig,
}

impl Servo {
    const PULSE_CYCLE: Duration = Duration::from_millis(20); 
    pub fn new_default(pin: u8) -> Servo  {
        return Servo::new_servo_from_config(ServoConfig::new_default(), pin);
    }

    pub fn new_servo_from_config(cfg: ServoConfig, pin: u8) -> Servo {
        let gpio = match Gpio::new() {
            Ok(g) => g,
            Err(e) => panic!("{:?}", e),
        };

        let gpio_pin = match gpio.get(pin) {
            Ok(gp) => gp,
            Err(e) => panic!("{:?}", e),
        }; 

        let mut output_pin = gpio_pin.into_output_low();
        output_pin.set_pwm(cfg.cycle, cfg.width);

        return Servo {
            pin: Some(output_pin),
            config: cfg
        }
    }

    pub fn get_config(&self) -> &ServoConfig {
        return &self.config;
    }

    pub fn set_angle(&mut self, a: u16) -> Result<(), ServoError> {
        let width = ServoConfig::calc_width_from_angle(a);

        if self.pin.is_none() {
            return Err(ServoError::PinNotSet);
        }

        let pin = self.pin.as_mut().unwrap();
        let cfg = ServoConfig {
            cycle: Duration::from_millis(20),
            width: width,
            angle: a
        };

        self.config = cfg;
        pin.set_pwm(self.config.cycle, self.config.width);

        return Ok(());
    }

    pub fn get_pulse_width(self) -> Duration {
        return self.config.width;
    }

    pub fn get_angle(self) -> u16 {
        return self.config.angle;
    }

    pub fn test_range(pin: u8, low: Duration, up: Duration) {
        let gpio = match Gpio::new() {
            Ok(g) => g,
            Err(e) => panic!("{:?}", e),
        };

        let gpio_pin = match gpio.get(pin) {
            Ok(gp) => gp,
            Err(e) => panic!("{:?}", e),
        }; 

        let mut output_pin = gpio_pin.into_output_low();
        let mut cur = low;
        let cycle = Servo::PULSE_CYCLE;
        while cur < up {
            cur = cur + Duration::from_micros(100);
            sleep(Duration::from_millis(500));
            output_pin.set_pwm(cycle, cur);
        }
    }
}

pub enum ServoError {
    PinNotSet,
}

pub struct ServoConfig {
    cycle: Duration,
    width: Duration,
    angle: u16
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

    pub fn new_default() -> ServoConfig {
        return ServoConfig {
            cycle: Duration::from_millis(20),
            width: Duration::from_micros(1500),
            angle: 90,
        }
    }

    pub fn new_config_from_width(w: Duration) -> ServoConfig {
        return ServoConfig::new(w);
    }

    pub fn new_config_from_angle(a: u16) -> ServoConfig {
        let width = ServoConfig::calc_width_from_angle(a);
        return ServoConfig::new(width);
    }

    fn calc_width_from_angle(a: u16) -> Duration {
        const MAXANGLE: u16 = 180;
        const MIN: f32 = 1000.0;
        const MAX: f32 = 2000.0;

        if a > MAXANGLE {
            panic!("angle can be at most 180");
        }

        let range: f32 = MAX - MIN;
        let prcnt: f32 = (a as f32) / (MAXANGLE as f32); 
        let scalar: f32 = prcnt * (range as f32);
        let width: u64 = (MIN as u64) + (scalar as u64);

        return Duration::from_micros(width);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_angle_calc() {
        let cfg = ServoConfig::new_config_from_angle(180);
        assert_eq!(cfg.width, Duration::from_micros(2000)); 
    }

    #[test]
    fn test_90_angle_calc() {
        let cfg = ServoConfig::new_config_from_angle(90);
        assert_eq!(cfg.width, Duration::from_micros(1500)); 
    }

    #[test]
    fn test_min_angle_calc() {
        let cfg = ServoConfig::new_config_from_angle(0);
        assert_eq!(cfg.width, Duration::from_micros(1000));
    }

    #[test]
    fn test_arbirary_angles() {
        let cfg = ServoConfig::new_config_from_angle(52);
        assert_eq!(cfg.width, Duration::from_micros(1288));
    }

    #[test]
    fn test_arbirary_angles2() {
        let cfg = ServoConfig::new_config_from_angle(134);
        assert_eq!(cfg.width, Duration::from_micros(1744));
    }
}
