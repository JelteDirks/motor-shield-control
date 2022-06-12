use rppal::gpio::{OutputPin, Gpio, Error as GpioError};
use core::time::Duration;
use std::thread::sleep;

/// The motor structure.
pub struct Motor {
    /// The pin of the motor.
    pub pin: Option<OutputPin>, 
    /// The direction of the motor, as an enum.
    direction: Direction,
    /// The status of the motor, for logging purposes.
    status: Status,
}

impl Motor {
    /// Creates a new motor without a pin set.
    pub fn new() -> Motor {
        println!("created new motor");
        return Motor {
            pin: None,
            direction: Direction::Clockwise,
            status: Status::Idle,
        }
    }

    /// Tests a range of a motor using pulse width and cycle.
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

    /// Returns whether or not the motor is running. This uses the enum that is
    /// created when the motor is created. The best way to see the status of 
    /// the motor is by extracting the information out of the pin.
    pub fn is_running(&self) -> bool {
        println!("check if motor is running: {:?} ", self.status);
        match self.status {
            Status::PWM => return true,
            Status::Idle => return false,
            Status::Running => return true,
        }
    }

    /// Inverts the direction of a motor. Keep in mind that this only sets the
    /// direction enum of the motor.
    pub fn invert_direction(&mut self) {
        println!("switching direction");
        match self.get_direction() {
            Direction::Clockwise => self.set_direction(Direction::Counterclockwise),
            Direction::Counterclockwise => self.set_direction(Direction::Clockwise),
        };
    }

    /// Sets the motor to a specified direction.
    pub fn set_direction(&mut self, d: Direction) {
        println!("setting direction to {:?}", d);
        self.direction = d;
    }

    /// Sets the pin of this motor.
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

    /// Returns the directions of this motor.
    pub fn get_direction(&self) -> Direction {
        println!("retrieving direction");
        return self.direction;
    }

    /// Returns the status of this board.
    pub fn get_status(&self) -> Status {
        println!("getting status");
        return self.status;
    }

    /// Sets the status of this board.
    pub fn set_status(&mut self, s: Status) {
        println!("set status to {:?}", s);
        self.status = s;
    }

    /// Starts this motor using the given configuration. The pin has to be set
    /// for this. If the configuration contains `full=true`, the motor will be
    /// run at full speed regardless of the pusle width settings.
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

    /// Stops the motor from running immediately.
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

#[derive(Debug)]
pub struct MotorConfig {
    cycle: Duration,
    width: Duration,
    full: bool
}

/// Motor configuration structure.
impl MotorConfig {
    /// Returns a new config with the cycle and width set.
    pub fn new_pwm(c: Duration, w: Duration) -> MotorConfig {
        return MotorConfig {
            cycle: c,
            width: w,
            full: false,
        }
    }

    /// Rerturns a new config with the full property set to true.
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
