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

    pub fn is_running(mut self) -> bool {
        match self.status {
            Status::Running => return true,
            _ => return false
        }
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
}
