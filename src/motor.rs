use rppal::gpio::{OutputPin, Gpio, Error as GpioError};
use core::time::Duration;
use std::thread::sleep;

pub struct Motor {
    pub pin: Option<OutputPin>, 
    direction: Direction,
    status: Status,
}

impl Motor {
    pub fn new() -> Motor {
        println!("created new motor");
        return Motor {
            pin: None,
            direction: Direction::Clockwise,
            status: Status::Idle,
        }
    }

    pub fn test_range(&mut self, cycle: Duration, low: Duration, up: Duration, step: Duration) {
        println!("testing motor pwm range");
        let output_pin: &mut OutputPin = match self.pin.as_mut() {
            Some(g) => g,
            None => panic!("pin for this motor is not set, can not test range"),
        };
        let mut cur = low;
        while cur < up {
            println!("cycle={:?} width={:?}", cycle, cur);
            output_pin.set_pwm(cycle, cur);
            sleep(Duration::from_millis(500));
            cur += step;
        }
    }

    pub fn is_running(&self) -> bool {
        println!("check if motor is running: {:?} ", self.status);
        match self.status {
            Status::PWM => return true,
            Status::Idle => return false,
            Status::Running => return true,
        }
    }

    pub fn invert_direction(&mut self) {
        println!("switching direction");
        match self.get_direction() {
            Direction::Clockwise => self.set_direction(Direction::Counterclockwise),
            Direction::Counterclockwise => self.set_direction(Direction::Clockwise),
        };
    }

    pub fn set_direction(&mut self, d: Direction) {
        println!("setting direction to {:?}", d);
        self.direction = d;
    }

    pub fn set_pin(&mut self, p: u8) -> Result<(), GpioError> {
        println!("setting motor pin to: {:?}", p);
        let gpio = Gpio::new();
        let gpio_pin = match gpio {
            Ok(gp) => gp,
            Err(e) => panic!("{:?}", e),
        };

        match gpio_pin.get(p) {
            Ok(pin) => self.pin = Some(pin.into_output()),
            Err(e) => panic!("{:?}", e),
        };

        println!("pin is set");

        return Ok(());
    }

    pub fn get_direction(&self) -> Direction {
        println!("retrieving direction");
        return self.direction;
    }

    pub fn get_status(&self) -> Status {
        println!("getting status");
        return self.status;
    }

    pub fn set_status(&mut self, s: Status) {
        println!("set status to {:?}", s);
        self.status = s;
    }

    pub fn start(&mut self, mc: MotorConfig) -> Result<(), MotorError> {
        if self.pin.is_none() {
            return Err(MotorError::PinNotSet);
        }

        let pin = self.pin.as_mut().unwrap();

        println!("starting motor");

        if mc.full {
            println!("full speed");
            pin.set_high();
            println!("pin is set to high: {:?}", pin.is_set_high());
            self.status = Status::Running;
            return Ok(());
        }

        println!("pwm cycle: {:?} width: {:?}", mc.cycle, mc.width);
        pin.set_pwm(mc.cycle, mc.width);
        self.status = Status::PWM;

        return Ok(());
    }

    pub fn stop(&mut self) -> Result<(), MotorError> {
        if self.pin.is_none() {
            return Err(MotorError::PinNotSet);
        }

        println!("stopping motor");

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

#[derive(Debug, Clone, Copy)]
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
