use rppal::gpio::{OutputPin, Gpio};
use std::time::Duration;
use std::thread::sleep;

/// The servo struct. This is specifically written for the Tower Pro SG90.
pub struct Servo {
    /// The pin that is used to address the servo.
    pin: Option<OutputPin>,
    /// The configuration that the servo is currently operating in.
    config: ServoConfig,
}

impl Servo {

    /// Returns a new servo from scratch. Only the pin number is required to
    /// create a new servo.
    pub fn new_default(pin: u8) -> Servo  {
        return Servo::new_servo_from_config(ServoConfig::new_default(), pin);
    }

    /// Returns a new servo from a configuration. A pin should also be provided.
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

    /// Rteurns a reference to the configuration that this servo is currently 
    /// using to operate.
    pub fn get_config(&self) -> &ServoConfig {
        return &self.config;
    }

    /// Sets the angle of this servo to the specified degress. The static
    /// method `calc_width_from_angle()` on `ServoConfig` is used to calculate
    /// the pulse width that is needed to reach the angle.
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
        println!("set servo to: cycle={:?} width={:?}", self.config.cycle, self.config.width);
        pin.set_pwm(self.config.cycle, self.config.width);

        return Ok(());
    }

    /// Returns the duration of the pulse width of this servo.
    pub fn get_pulse_width(self) -> Duration {
        return self.config.width;
    }

    /// Returns the angle that this servo is currently in.
    pub fn get_angle(self) -> u16 {
        return self.config.angle;
    }

    /// Tests the servo range using a lower and upper bound on the pulse width.
    /// A pin should also be specified.
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
        let cycle = ServoConfig::PULSE_CYCLE;
        while cur <= up {
            println!("testing servo range width={:?} and cycle{:?}", cur, cycle);
            sleep(Duration::from_millis(500));
            output_pin.set_pwm(cycle, cur);
            cur = cur + Duration::from_micros(100);
        }

        output_pin.set_low();
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
    /// The minimum pulse width of the servo, used to calculate angles.
    const MIN: f32 = 600.0; 
    /// The maximum pulse width of the servo, used to calculate angles.
    const MAX: f32 = 2550.0;
    /// The maximum angle that the servo supports.
    const MAX_ANGLE: u16 = 180; 
    /// The pulse cycle of the servo. This is made for servos that use a 
    /// frequency of 50 Herz or 20 milliseconds.
    const PULSE_CYCLE: Duration = Duration::from_millis(20); 

    /// Returns a new configuration with a specified pulse width. The cycle
    /// is set to the 50 hz default cycle length.
    fn new(width: Duration) -> ServoConfig {
        if width < Duration::from_micros(ServoConfig::MIN as u64) {
            panic!("pulse width should be at least 0.4ms");
        }
 
        if width > Duration::from_micros(ServoConfig::MAX as u64) {
            panic!("pulse width should be at most 2.6ms");
        }

        return ServoConfig {
            cycle: ServoConfig::PULSE_CYCLE,
            width: width,
            angle: 0
        }
    }

    /// Returns a new default configuration with the default angle. This is 
    /// currently set locally to 90 degrees.
    pub fn new_default() -> ServoConfig {
        const default_angle: u16 = 90;
        return ServoConfig {
            cycle: ServoConfig::PULSE_CYCLE,
            width: ServoConfig::calc_width_from_angle(default_angle),
            angle: default_angle,
        }
    }

    /// Returns a new configuration based on the specified pulse width.
    pub fn new_config_from_width(w: Duration) -> ServoConfig {
        return ServoConfig::new(w);
    }

    /// Returns a new configuration base on the specified angle.
    pub fn new_config_from_angle(a: u16) -> ServoConfig {
        let width = ServoConfig::calc_width_from_angle(a);
        return ServoConfig::new(width);
    }

    /// Returns the duration of a pulse width calculated from the specified
    /// angle.
    fn calc_width_from_angle(a: u16) -> Duration {
        if a > ServoConfig::MAX_ANGLE {
            panic!("angle can be at most 180");
        }

        // the range over which the servo can operate
        let range: f32 = ServoConfig::MAX - ServoConfig::MIN;
        // the percentage of width over the total range
        let prcnt: f32 = (a as f32) / (ServoConfig::MAX_ANGLE as f32); 
        // the scalar that should go on top of the minimum value
        let scalar: f32 = prcnt * (range as f32);
        // the final width of the pulse
        let width: u64 = (ServoConfig::MIN as u64) + (scalar as u64);

        return Duration::from_micros(width);
    }
}

