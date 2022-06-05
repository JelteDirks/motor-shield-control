use crate::motor::{Motor, MotorError, Direction};

pub struct AMSBoard {    
    dirSer: Option<u8>,
    dirClk: Option<u8>,
    dirLat: Option<u8>,
    motors: [Option<Motor>; 4],
    _type: BoardType,
}
  
impl AMSBoard {
    pub fn new(t: BoardType) -> AMSBoard {
        return AMSBoard {
            dirSer: None,
            dirClk: None,
            dirLat: None,
            motors: [None, None, None, None],
            _type: t,
        }
    }

    pub fn get_motor(&mut self, n: usize) -> Result<&mut Motor, MotorError> {
        let motorOpt = &mut self.motors[n - 1];
        match motorOpt {
            Some(motor) => return Ok(motor),
            None => return Err(MotorError::MotorNotFound),
        }
    }

    pub fn set_motor(&mut self, m: Motor, n: usize) -> Result<bool, MotorError> {
        if n < 1 || n > 4 {
            return Err(MotorError::MotorIndexOutOfBounds);
        }

        self.motors[n - 1] = Some(m);

        return Ok(true);
    }

    pub fn set_shift_register(&mut self, ser: u8, clk: u8, lat: u8) {
        self.dirSer = Some(ser);
        self.dirClk = Some(clk);
        self.dirLat = Some(lat);
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
