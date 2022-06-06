use rppal::gpio::{OutputPin, Gpio, Error as GpioError};
use core::time::Duration;

pub struct Motor {
    pub pin: Option<OutputPin>, 
    direction: Direction,
    status:Status,
}

impl Motor {
    pub fn new() -> Motor {
        return Motor {
            pin: None,
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

    pub fn get_direction(&self) -> Direction {
        return self.direction;
    }

    pub fn get_status(&self) -> Status {
        return self.status;
    }

    pub fn set_status(&mut self, s: Status) {
        self.status = s;
    }

    pub fn start(&mut self, mc: MotorConfig) -> Result<(), MotorError> {
        if self.pin.is_none() {
            return Err(MotorError::PinNotSet);
        }

        let pin = self.pin.as_mut().unwrap();

        if mc.full {
            pin.set_high();
            self.status = Status::Running;
            return Ok(());
        }

        pin.set_pwm(mc.cycle, mc.width);
        self.status = Status::PWM;
        
        return Ok(());
    }

    pub fn stop(&mut self) -> Result<(), MotorError> {
        if self.pin.is_none() {
            return Err(MotorError::PinNotSet);
        }

        let pin: &mut OutputPin = self.pin.as_mut().unwrap();
        pin.set_low();
        self.status = Status::Idle;

        return Ok(());
    }
}

pub struct MotorConfig {
    cycle: Duration,
    width: Duration,
    full: bool
}

impl MotorConfig {
    pub fn new_pwm(c: Duration, w: Duration) -> MotorConfig {
        return MotorConfig {
            cycle: c,
            width: w,
            full: false,
        }
    }

    pub fn new_full() -> MotorConfig {
        return MotorConfig {
            cycle: Duration::from_millis(100),
            width: Duration::from_millis(100),
            full: true,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Clockwise,
    Counterclockwise
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    PWM,
    Idle,
    Running,
}

#[derive(Debug)]
pub enum MotorError {
    PinNotSet,
    SpeedIsZero,
    MotorNotFound,
    MotorIndexOutOfBounds,
    PWMDurationTooHigh,
}
