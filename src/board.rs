use crate::motor::{Motor, MotorError};

pub struct AMSBoard {    
    dirSer: u8,
    dirClk: u8,
    dirLat: u8,
    motors: [Option<Motor>; 4],
  }
  
  impl AMSBoard {
    pub fn get_motor(&self, n: usize) -> Result<&Motor, MotorError> {
        let motorOpt = &self.motors[n];
        match motorOpt {
            Some(motor) => Ok(motor),
            None => Err(MotorError::MotorNotFound),
        }
    }
  }