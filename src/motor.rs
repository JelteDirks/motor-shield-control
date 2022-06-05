pub struct Motor {
    pin: u8,
    pwm: u8,
    direction: Direction,
    status:Status,
}

impl Motor {
    pub fn new() -> Motor {
        return Motor {
            pin: 0,
            pwm: 100,
            direction: Direction::Clockwise,
            status: Status::Idle,
        }
    }

    pub fn is_running(&self) -> bool {
        match self.status {
            Status::Running => return true,
            Status::PWM => return true,
            _ => return false,
        }
    }

    pub fn set_direction(&mut self, d: Direction) {
        self.direction = d;
    }

    pub fn set_pin(&mut self, p: u8) {
        self.pin = p;
    }

    pub fn set_pwm(&mut self, pwm: u8) {
        self.pwm = pwm;
    }
}

pub enum Direction {
    Clockwise,
    Counterclockwise
}

pub enum Status {
    Running,
    PWM,
    Idle
}

pub enum MotorError {
    PinNotSet,
    SpeedIsZero,
    MotorNotFound,
    MotorIndexOutOfBounds,
}
