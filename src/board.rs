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
}
