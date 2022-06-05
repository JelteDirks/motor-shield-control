use rppal::gpio::{OutputPin, Gpio, Error as GpioError};
use core::time::Duration;

pub struct Motor {
    pub pin: Option<OutputPin>, 
    pub pwm_cycle: Duration,
    pub pulse_width: Duration,
    direction: Direction,
    status:Status,
}

impl Motor {
    pub fn new() -> Motor {
        return Motor {
            pin: None,
            pwm_cycle: Duration::from_millis(100),
            pulse_width: Duration::from_millis(100),
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

    pub fn get_pwm_cycle(&self) -> Duration {
        return self.pwm_cycle;
    }

    pub fn get_pulse_width(&self) -> Duration {
        return self.pulse_width;
    }

    pub fn set_pwm_cycle(&mut self, pp: Duration) {
        self.pwm_cycle = pp;
    }

    pub fn set_pulse_width(&mut self, pd: Duration) {
        self.pulse_width = pd;
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
