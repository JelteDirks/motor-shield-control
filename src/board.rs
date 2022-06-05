use crate::motor::{Motor, MotorError};

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

    pub fn get_motor(&self, n: usize) -> Result<&Motor, MotorError> {
        let motorOpt = &self.motors[n];
        match motorOpt {
            Some(motor) => return Ok(motor),
            None => return Err(MotorError::MotorNotFound),
        }
    }

    pub fn set_motor(&mut self, m: Motor, n: usize) -> Result<bool, MotorError> {
        if n > 3 {
            return Err(MotorError::MotorIndexOutOfBounds);
        }

        self.motors[n] = Some(m);

        return Ok(true);
    }

    pub fn set_shift_register(&mut self, ser: u8, clk: u8, lat: u8) {
        self.dirSer = Some(ser);
        self.dirClk = Some(clk);
        self.dirLat = Some(lat);
    }
}


pub enum BoardType {
    BCM,
    Board
}
