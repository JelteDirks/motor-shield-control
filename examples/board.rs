use motor_shield_control::motor::*;
use motor_shield_control::board::*;
use motor_shield_control::servo::*;
use std::time::Duration;

fn main() {
    let mut board = AMSBoard::new(BoardType::BCM); // create a new board with board type BCM
    let mut motor = Motor::new(); // motor is created, but nothing is set yet
    motor.set_pin(16); // the pin has to be set explicitly

    /// The shift register pins have to be set explicitly.
    /// Take note that the order is important, the parameter order is as
    /// follows: serial, clock, latch.
    /// If this does not mean anything to you, please read the documentation.
    /// NOTE: not setting this will make the motors unable to change direction
    /// using code, and will give `Err` results!
    board.set_shift_register_pins(16, 17, 18);

    /// Once the motor is created, it still has to be added to the board. This
    /// can be done using `set_motor`, yet keep in mind. The number you provide
    /// to set motor is the motor slot that you have to use on the board. This
    /// is done to provide correct direction capabilities.
    board.set_motor(motor, 2);

    /// This starts the motor at full speed. Which means a pulse width equal
    /// to the pulse cycle. Only the number of the motor is needed.
    board.start_motor_full(2);

    board.stop_motor(2); // this will stop the motor entirely

    /// You can also start a motor with a pulse cycle and width.
    board.start_motor_pwm(2, Duration::from_millis(20), Duration::from_millis(10));

    /// Starting a motor using your own configuration for pulse width and cycle
    /// or full speed.
    /// Note that if you start a motor with full: false but you set the pwm
    /// cycle and width to the same duration, you effectively run the motor at
    /// full speed but if you check the motor status it will show that it runs
    /// as pwm.
    board.start_motor_config(2, MotorConfig::new_pwm(Duration::from_millis(20), Duration::from_millis(10)));
}
