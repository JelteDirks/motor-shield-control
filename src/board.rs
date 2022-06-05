use crate::motor::{Motor, MotorError, Direction};
use rppal::gpio::{OutputPin, Gpio, Pin, Error as GpioError};

pub struct AMSBoard {    
    dir_ser: Option<u8>,
    dir_clk: Option<u8>,
    dir_lat: Option<u8>,
    pin_ser: Option<OutputPin>,
    pin_clk: Option<OutputPin>,
    pin_lat: Option<OutputPin>,
    motors: [Option<Motor>; 4],
    _type: BoardType,
    directions: u8,
}
  
impl AMSBoard {
    pub fn new(t: BoardType) -> AMSBoard {
        return AMSBoard {
            dir_ser: None,
            dir_clk: None,
            dir_lat: None,
            pin_ser: None,
            pin_clk: None,
            pin_lat: None,
            motors: [None, None, None, None],
            _type: t,
            directions: 0,
        }
    }

    pub fn get_motor(&mut self, n: usize) -> Result<&mut Motor, MotorError> {
        let motor = &mut self.motors[n - 1];
        match motor {
            Some(m) => return Ok(m),
            None => return Err(MotorError::MotorNotFound),
        }
    }

    pub fn set_motor(&mut self, m: Motor, n: usize) -> Result<bool, MotorError> {
        if n < 1 || n > 4 {
            return Err(MotorError::MotorIndexOutOfBounds);
        }

        self.motors[n - 1] = Some(m);
        self.update_directions();

        return Ok(true);
    }

    pub fn set_shift_register(&mut self, ser: u8, clk: u8, lat: u8) {
        self.dir_ser = Some(ser);
        self.dir_clk = Some(clk);
        self.dir_lat = Some(lat);

        let gpio_res: Result<Gpio, GpioError> = Gpio::new(); 
        let gpio = match gpio_res {
            Ok(g) => g,
            Err(e) => panic!("{:?}", e),
        };

        let latch_pin = gpio.get(lat);
        let serial_pin = gpio.get(ser);
        let clock_pin = gpio.get(clk); 

        match clock_pin {
            Ok(p) => self.pin_clk = Some(p.into_output()),
            Err(e) => panic!("{:?}", e),
        };

        match latch_pin {
            Ok(p) => self.pin_lat = Some(p.into_output()),
            Err(e) => panic!("{:?}", e),
        };
        match serial_pin {
            Ok(p) => self.pin_ser = Some(p.into_output()),
            Err(e) => panic!("{:?}", e),
        };
    }

    fn calculate_directions(&self) -> u8 {
        let m1_dir: u8 = match &self.motors[0] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 4,
                Direction::Counterclockwise => 8
            },
            _ => 0
        };

        let m2_dir: u8 = match &self.motors[1] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 2,
                Direction::Counterclockwise => 16 
            },
            _ => 0
        };

        let m3_dir: u8 = match &self.motors[2] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 1,
                Direction::Counterclockwise => 64 
            },
            _ => 0
        };

        let m4_dir: u8 = match &self.motors[3] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 32,
                Direction::Counterclockwise => 128 
            },
            _ => 0
        };

        return m1_dir | m2_dir | m3_dir | m4_dir; 
    }

    pub fn get_directions(&self) -> u8 {
        return self.directions; 
    }
    
    fn update_directions (&mut self) {
        self.directions = self.calculate_directions();
    }

    pub fn change_motor_direction(&mut self, p: usize, d: Direction) -> Result<(), MotorError> {
        match &mut self.motors[p - 1] {
            Some(motor) => motor.set_direction(d),
            _ => return Err(MotorError::MotorNotFound),
        };
        self.update_directions();
        return Ok(());
    }

    pub fn change_motor_pwm(&mut self, p: usize, pwm: u8) -> Result<(), MotorError> {
        match &mut self.motors[p -1] {
            Some(motor) => motor.set_pwm(pwm),
            _ => return Err(MotorError::MotorNotFound),
        }
        return Ok(());
    }

    fn register_pins_are_valid(&self) -> bool {
        if self.pin_clk.is_none() {
            return false;
        }
        
        if self.pin_ser.is_none() {
            return false;
        }

        if self.pin_lat.is_none() {
            return false;
        }

        return true;
    }

    fn update_shift_register(&mut self) {
        if !self.register_pins_are_valid() {
            panic!("register pins are not set correctly");
        }

        let latch = self.pin_lat.as_mut().unwrap();
        let serial = self.pin_ser.as_mut().unwrap();
        let clock = self.pin_clk.as_mut().unwrap();

        latch.set_low();

        let mut b: u16 = 0b1;
        while b == 128 {
            clock.set_low();
            let c: u16 = b & (self.directions as u16);
            if c == b {
                serial.set_high();
            } else {
                serial.set_low();
            }
            clock.set_high();
            b = b << 1;
        }        

        latch.set_high();
    }

    pub fn update_motors(&mut self) -> Result<(), BoardError> {
        if self.pin_lat.is_none() {
            return Err(BoardError::LatchPinNotSet); 
        }

        if self.pin_ser.is_none() {
            return Err(BoardError::SerialPinNotSet);
        }

        if self.pin_clk.is_none() {
            return Err(BoardError::ClockPinNotSet);
        }

        self.update_directions();
        self.update_shift_register();

        // todo!("push values into shift register and set motor pwm values");

        return Ok(());
    }
}

pub enum BoardError {
    LatchPinNotSet,
    SerialPinNotSet,
    ClockPinNotSet,
}

pub enum BoardType {
    BCM,
    Board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_m1direction_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let motor = Motor::new();
        board.set_motor(motor, 1);
        let direction: u8 = board.calculate_directions();
        assert_eq!(4, direction);
        match board.get_motor(1) {
            Ok(m) => m.set_direction(Direction::Counterclockwise),
            Err(e) => panic!("{:?}",e),
        }
        let direction: u8 = board.calculate_directions();
        assert_eq!(8, direction);
    }
    
    #[test]
    fn set_m2direction_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor = Motor::new();
        board.set_motor(motor, 2);
        let direction: u8 = board.calculate_directions();
        assert_eq!(2, direction);
        match board.get_motor(2) {
            Ok(m) => m.set_direction(Direction::Counterclockwise),
            Err(e) => panic!("{:?}",e),
        }
        let direction: u8 = board.calculate_directions();
        assert_eq!(16, direction);
    }

    #[test]
    fn set_m3direction_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor = Motor::new();
        board.set_motor(motor, 3);
        let direction: u8 = board.calculate_directions();
        assert_eq!(1, direction);
        match board.get_motor(3) {
            Ok(m) => m.set_direction(Direction::Counterclockwise),
            Err(e) => panic!("{:?}",e),
        }
        let direction: u8 = board.calculate_directions();
        assert_eq!(64, direction);
    }
    
    #[test]
    fn set_m4direction_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor = Motor::new();
        board.set_motor(motor, 4);
        let direction: u8 = board.calculate_directions();
        assert_eq!(32, direction);
        match board.get_motor(4) {
            Ok(m) => m.set_direction(Direction::Counterclockwise),
            Err(e) => panic!("{:?}",e),
        }
        let direction: u8 = board.calculate_directions();
        assert_eq!(128, direction);
    }

    #[test]
    fn composite_direction_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor4 = Motor::new();
        board.set_motor(motor4, 4);
        let mut motor2 = Motor::new();
        board.set_motor(motor2, 2);
        let direction: u8 = board.calculate_directions();
        assert_eq!(0b00100010, direction);
    }
    
    #[test]
    fn all_motor_clockwise_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor1 = Motor::new();
        board.set_motor(motor1, 1);
        let mut motor2 = Motor::new();
        board.set_motor(motor2, 2);
        let mut motor3 = Motor::new();
        board.set_motor(motor3, 3);
        let mut motor4 = Motor::new();
        board.set_motor(motor4, 4);
        let direction: u8 = board.calculate_directions();
        assert_eq!(0b00100111, direction);
    }
    
    #[test]
    fn all_motor_cclockwise_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor1 = Motor::new();
        motor1.set_direction(Direction::Counterclockwise);
        board.set_motor(motor1, 1);
        let mut motor2 = Motor::new();
        motor2.set_direction(Direction::Counterclockwise);
        board.set_motor(motor2, 2);
        let mut motor3 = Motor::new();
        motor3.set_direction(Direction::Counterclockwise);
        board.set_motor(motor3, 3);
        let mut motor4 = Motor::new();
        motor4.set_direction(Direction::Counterclockwise);
        board.set_motor(motor4, 4);
        let direction: u8 = board.calculate_directions();
        assert_eq!(0b11011000, direction);
    }
}

