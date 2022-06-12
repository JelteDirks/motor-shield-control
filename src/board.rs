use crate::motor::{Motor, MotorError, Direction, MotorConfig};
use rppal::gpio::{OutputPin, Gpio, Error as GpioError};
use core::time::Duration;
use std::matches;

/// # The AMSBoard structure
pub struct AMSBoard {
    /// the serial pin
    pin_ser: Option<OutputPin>,
    /// the clock pin
    pin_clk: Option<OutputPin>,
    /// the latch pin
    pin_lat: Option<OutputPin>,
    /// the list that holds the motors
    motors: [Option<Motor>; 4],
    /// the board type, currently does nothing
    _type: BoardType,
    /// the byte that holds the directions of the motors
    directions: u8,
}
  
impl AMSBoard {
    /// Creates a new AMSBoard. The `BoardType` parameter is required but 
    /// currently serves no functionality.
    pub fn new(t: BoardType) -> AMSBoard {
        println!("creating new board");
        return AMSBoard {
            pin_ser: None,
            pin_clk: None,
            pin_lat: None,
            motors: [None, None, None, None],
            _type: t,
            directions: 0,
        }
    }

    /// This function calculates the directions of all the motors. It relies
    /// on the fact that motor 1 is actually in slot M1 on the motor shield,
    /// since these directions are pushed to a shift register which have fixed
    /// data lines that decide the direction of the M slots. The function gets
    /// all motor directions and does a bitwise OR to include every direction
    /// in the byte. This is done to prevent the issue where only setting one
    /// motors direction will make all other motors stall.
    fn calculate_directions(&self) -> u8 {
        println!("calculating directions of motors");
        let m1_dir: u8 = match &self.motors[0] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 4,
                Direction::Counterclockwise => 8
            },
            _ => 0
        };
        println!("motor 1: {:#010b}", m1_dir);

        let m2_dir: u8 = match &self.motors[1] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 2,
                Direction::Counterclockwise => 16 
            },
            _ => 0
        };

        println!("motor 2: {:#010b}", m2_dir);
        
        let m3_dir: u8 = match &self.motors[2] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 1,
                Direction::Counterclockwise => 64 
            },
            _ => 0
        };
        
        println!("motor 3: {:#010b}", m3_dir);

        let m4_dir: u8 = match &self.motors[3] {
            Some(m) => match m.get_direction(){
                Direction::Clockwise => 32,
                Direction::Counterclockwise => 128 
            },
            _ => 0
        };
        println!("motor 4: {:#010b}", m4_dir);

        println!("total direction: {:#010b}", m1_dir | m2_dir | m3_dir | m4_dir);

        return m1_dir | m2_dir | m3_dir | m4_dir; 
    }

    /// An internal function to update the directions of this board. It
    /// recalculates directions and stores them in the interal directions
    /// property.
    fn update_directions (&mut self) {
        println!("calculating internal directions");
        self.directions = self.calculate_directions();
    }

    /// Checks if the register pins are valid. If any of them is `None`, the
    /// function will return false.
    fn register_pins_are_valid(&self) -> bool {
        println!("checking if pins are valid");
        if self.pin_clk.is_none() {
            return false;
        }
        
        if self.pin_ser.is_none() {
            return false;
        }

        if self.pin_lat.is_none() {
            return false;
        }

        println!("pins are valid!");

        return true;
    }

    /// This function updates the shift register. It does this by looking at 
    /// the directions property of the AMSBoard struct, and iterating over each
    /// bit from MSB to LSB, pushing it onto the register. The latch is then
    /// opened and the byte is stored into the memory.
    fn update_shift_register(&mut self) -> Result<(), BoardError> {
        if !self.register_pins_are_valid() {
            return Err(BoardError::RegisterPinNotSet);
        }

        let latch = self.pin_lat.as_mut().unwrap();
        let serial = self.pin_ser.as_mut().unwrap();
        let clock = self.pin_clk.as_mut().unwrap();

        println!("\tpushing to shift register. latch={:?}, serial={:?} clock={:?}", latch.pin(), serial.pin(), clock.pin());

        latch.set_low(); // set the latch low before pushing
        println!("\tset latch low");
        let mut b: u16 = 128; // initiate the MSB as the first bit to be pushed
        while b != 0 { // continue as long as we still have to push a bit
            println!("setting bit position {:#010b}", b);
            println!("\tset clock low");
            clock.set_low(); // set the clock low, we need a rising edge to push
            let c: u16 = b & (self.directions as u16); // bitwise AND with the bit to push
            if c == b { // if the directions had a 1 on that bit, it is still 1
                println!("\tset serial high");
                serial.set_high(); // this means that this push should be high
            } else { // otherwise it was a 0
                println!("\tset serial low");
                serial.set_low(); // so we push a low
            }
            println!("\tset clock high");
            clock.set_high(); // set the clock high, a rising edge will push the data
            b = b >> 1; // shift the bit to push to the right
        }        

        println!("\tdirections are set to {:#010b}", self.directions);
        println!("\tset latch high");
        latch.set_high(); // once all bits are pushed, store in memory by opening the latch

        println!("\tlatch.is_high={:?}\n\tclock.is_high={:?}\n\tserial.is_high={:?}", latch.is_set_high(), clock.is_set_high(), serial.is_set_high());

        return Ok(());
    }

    /// Starts a motor using a configuration. Used internally to unify the 
    /// process of starting a motor over all public accessors.
    fn start_motor(&mut self, n: usize, cfg: MotorConfig) -> Result<(), BoardError> {
        if n < 1 || n > 4 {
            return Err(BoardError::MotorError(MotorError::MotorIndexOutOfBounds));
        }

        if self.motors[n - 1].is_none() {
            return Err(BoardError::MotorError(MotorError::MotorNotFound));
        }
        
        self.update_shift_register(); // first update the shift register
        let motor: &mut Motor = self.motors[n - 1].as_mut().unwrap(); // retrieve the motor
        
        println!("starting motor");
        motor.start(cfg); // start the motor with the configuration

        return Ok(());
    }
    
    /// Returns a mutable reference to the motor that is requested. If there
    /// is no motor found, a MotorError is returned.
    pub fn get_motor(&mut self, n: usize) -> Result<&mut Motor, MotorError> {
        if n < 1 || n > 4 {
            return Err(MotorError::MotorIndexOutOfBounds);
        }

        let motor = &mut self.motors[n - 1];
        match motor {
            Some(m) => return Ok(m),
            None => return Err(MotorError::MotorNotFound),
        }
    }

    /// Sets a motor slot of the board. Use the M slots on the board to 
    /// indicate motors, since the directions are tied to these slots.
    pub fn set_motor(&mut self, m: Motor, n: usize) -> Result<bool, MotorError> {
        if n < 1 || n > 4 {
            return Err(MotorError::MotorIndexOutOfBounds);
        }

        println!("set motor number {:?}", n);
        self.motors[n - 1] = Some(m);
        self.update_directions();

        return Ok(true);
    }

    /// Set the shift register pins of the board. Without the shift register
    /// there is no addressing motors since the shift register is reset on 
    /// every restart, thus all motors will stall.
    pub fn set_shift_register_pins(&mut self, ser: u8, clk: u8, lat: u8) {
        let gpio_res: Result<Gpio, GpioError> = Gpio::new(); 
        let gpio = match gpio_res {
            Ok(g) => g,
            Err(e) => panic!("{:?}", e),
        };

        let latch_pin = gpio.get(lat);
        let serial_pin = gpio.get(ser);
        let clock_pin = gpio.get(clk); 
        
        println!("setting shift register pins latch={:?} clock={:?} serial={:?}", lat, clk, ser);
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

    /// Returns the directions of the board. This is the byte that contains
    /// the directions of all motors. Note that they are not calculated when
    /// you call this method, so this has to be done by the user.
    pub fn get_directions(&self) -> u8 {
        println!("retrieving directions");
        return self.directions; 
    }

    /// Inverts the direction of the given motor.
    pub fn invert_motor_direction(&mut self, m: usize) {
        println!("inverting motor directions");
        match &mut self.motors[m - 1] {
            Some(motor) => motor.invert_direction(),
            None => println!("no motor set for {:?}", m),
        };

        self.update_directions();
        self.update_shift_register();
    }
    
    /// Changes the specified motor to the specified direction.
    pub fn change_motor_direction(&mut self, p: usize, d: Direction) -> Result<(), MotorError> {
        println!("change motor directions of motor {:?} to {:?}", p, d);
        match &mut self.motors[p - 1] {
            Some(motor) => {
                motor.set_direction(d);
                self.update_directions();
                self.update_shift_register();
                return Ok(());
            },
            _ => return Err(MotorError::MotorNotFound),
        };
    }

    /// Starts a specified motor with a specified configuration.
    pub fn start_motor_config(&mut self, n: usize, mc: MotorConfig) -> Result<(), BoardError> {
        println!("start motor {:?} with config {:?}", n, mc);
        return self.start_motor(n, mc);
    }

    /// Starts a specified motor using a pulse cycle and pusle width.
    pub fn start_motor_pwm(&mut self, n: usize, cycle: Duration, width: Duration) -> Result<(), BoardError> {
        println!("start motor {:?} with pwm cycle {:?} and width {:?}", n, cycle, width);
        return self.start_motor(n, MotorConfig::new_pwm(cycle, width));
    }

    /// Starts a specified motor on full speed.
    pub fn start_motor_full(&mut self, n: usize) -> Result<(), BoardError> {
        println!("start motor {:?} at full speed", n);
        return self.start_motor(n, MotorConfig::new_full());
    }

    /// Stops a specified motor gracefully.
    pub fn stop_motor(&mut self, n: usize) -> Result<(), BoardError> { 
        if n < 1 || n > 4 {
            return Err(BoardError::MotorError(MotorError::MotorIndexOutOfBounds));
        }

        if self.motors[n - 1].is_none() {
            return Err(BoardError::MotorError(MotorError::MotorNotFound));
        }

        let motor: &mut Motor = self.motors[n - 1].as_mut().unwrap();
        println!("stopping motor");
        motor.stop();

        return Ok(());
    }

    pub fn test_motor_range(&mut self, n: usize, cycle:Duration, low:Duration, up:Duration, step:Duration) {
        if n < 1 || n > 4 {
            panic!("index out of bounds");
        }

        if self.motors[n - 1].is_none() {
            panic!("motor is not set");
        }

        let motor: &mut Motor = self.motors[n - 1].as_mut().unwrap();
        println!("testing motor {:?} range", n);
        motor.test_range(cycle, low, up, step);
    }
}

pub enum BoardError {
    RegisterPinNotSet,
    MotorError(MotorError),
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

    #[test]
    fn motor_is_running_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor = Motor::new();

        motor.set_pin(16);
        board.set_shift_register_pins(16, 20, 19);
        board.set_motor(motor, 1);
        board.start_motor_full(1);
        match board.get_motor(1) {
            Ok(m) => assert_eq!(m.is_running(), true),
            Err(e) => panic!("{:?}", e),
        }

        board.stop_motor(1);
        match board.get_motor(1) {
            Ok(m) => assert_eq!(m.is_running(), false),
            Err(e) => panic!("{:?}", e),
        }

        board.start_motor_pwm(1, Duration::from_millis(20), Duration::from_millis(10));
        match board.get_motor(1) {
            Ok(m) => assert_eq!(m.is_running(), true),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn start_wrong_motor_error_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        assert!(board.start_motor_full(1).is_err());
    }

    #[test]
    fn get_wrong_motor_error_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        assert!(board.get_motor(1).is_err());
    }

    #[test]
    fn wrong_motor_index_error_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        assert!(board.get_motor(5).is_err());
        assert!(board.get_motor(0).is_err());

        let mut motor = Motor::new();
        board.set_motor(motor, 2);
        assert!(board.get_motor(2).is_ok());
    }

    #[test]
    fn test_motor_range() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor = Motor::new();
        motor.set_pin(16);
        board.set_motor(motor, 1);
        let cycle = Duration::from_millis(20);
        let low = Duration::from_millis(2);
        let up = Duration::from_millis(20);
        let step = Duration::from_millis(2);
        board.test_motor_range(1, cycle, low, up, step);
    }

    #[test]
    fn motor_direction_invert_test() {
        let mut board = AMSBoard::new(BoardType::BCM);
        let mut motor = Motor::new();
        motor.set_pin(16);
        board.set_motor(motor, 1);
        board.invert_motor_direction(1);
        let m: &Motor = board.get_motor(1).unwrap();
        assert!(matches!(m.get_direction(), Direction::Counterclockwise));
    }
}

