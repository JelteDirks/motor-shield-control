

fn main() {
    println!("Hello, world!");
    let mut board = AMSBoard::new();
    board.motor_1.gpio_pin = 13;
    board.shift_register.latch_pin = 16
}

pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub struct Motor {
    gpio_pin: u8,
    is_running: bool,
    direction: Direction,
}

pub struct AMSBoard {
    motor_1: Motor,
    motor_2: Motor,
    motor_3: Motor,
    motor_4: Motor,
    shift_register:ShiftRegister,
}

pub struct ShiftRegister {
    latch_pin: u8,
    clock_pin: u8,
    serial_pin: u8
}

impl ShiftRegister {
    pub fn validate(&self) -> bool {
        if self.latch_pin == 0 {
            panic!("latch pin is not set")
        }
        
        if self.clock_pin == 0 {
            panic!("clock pin is not set")
        }

        if self.serial_pin == 0 {
            panic!("serial pin is not set")
        }

        return true
    }
}
impl AMSBoard {
    pub fn new() -> AMSBoard {
        return AMSBoard {
            motor_1: Motor {
                gpio_pin: 0,
                is_running: false,
                direction: Direction::Clockwise,
            },
            motor_2: Motor {
                gpio_pin: 0,
                is_running: false,
                direction: Direction::Clockwise,
            },
            motor_3: Motor {
                gpio_pin: 0,
                is_running: false,
                direction: Direction::Clockwise,
            },
            motor_4: Motor {
                gpio_pin: 0,
                is_running: false,
                direction: Direction::Clockwise,
            },
            shift_register: ShiftRegister {
                latch_pin: 0,
                clock_pin:0,
                serial_pin: 0,
            }
        }
    }
}
