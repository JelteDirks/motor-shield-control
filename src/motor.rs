use rppal::gpio::{OutputPin, Gpio, Error as GpioError};
use core::time::Duration;

pub struct Motor {
    pin: Option<OutputPin>, 
    pwm_period: Duration,
    pwm_duration: Duration,
    direction: Direction,
    status:Status,
}

impl Motor {
    pub fn new() -> Motor {
        return Motor {
            pin: None,
            pwm_period: Duration::from_millis(100),
            pwm_duration: Duration::from_millis(100),
            direction: Direction::Clockwise,
            status: Status::Idle,
        }
    }

    pub fn is_running(&self) -> bool {
        match self.status {
            Status::PWM => return true,
            _ => return false,
        }
    }

    pub fn set_direction(&mut self, d: Direction) {
        self.direction = d;
    }

    pub fn set_pin(&mut self, p: u8) -> Result<(), GpioError> {
        let gpio = Gpio::new();
        let gpio_pin = match gpio {
            Ok(gp) => gp,
            Err(e) => panic!("{:?}", e),
        };

        match gpio_pin.get(p) {
            Ok(pin) => self.pin = Some(pin.into_output()),
            Err(e) => panic!("{:?}", e),
        };

        return Ok(());
    }

    pub fn set_pwm_period(&mut self, pp: Duration) {
        self.pwm_period = pp;
    }

    pub fn set_pwm_duration(&mut self, pd: Duration) {
        self.pwm_duration = pd;
    }

    pub fn get_direction(&self) -> Direction {
        return self.direction;
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Clockwise,
    Counterclockwise
}

#[derive(Debug)]
pub enum Status {
    PWM,
    Idle
}

#[derive(Debug)]
pub enum MotorError {
    PinNotSet,
    SpeedIsZero,
    MotorNotFound,
    MotorIndexOutOfBounds,
    PWMDurationTooHigh,
}
