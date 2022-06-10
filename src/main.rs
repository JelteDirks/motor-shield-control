use motor_shield_control::board::*;
use motor_shield_control::motor::*;
use motor_shield_control::servo::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("main started");

    let low = Duration::from_micros(500);
    let high = Duration::from_micros(2500);
    Servo::test_range(26, low, high);

    let mut servo = Servo::new_default(26);
    servo.set_angle(0);
    sleep(Duration::from_secs(2));
    
//    start_motor_full(1);
//    start_motor_full(2);
//    start_motor_full(3);
//    start_motor_full(4);

//    start_motor(1);
//    start_motor(2);
//    start_motor(3);
//    start_motor(4);

    println!("main finished");
}


fn start_motor(n: usize) {
    let mut board = AMSBoard::new(BoardType::BCM);
    let mut motor = Motor::new();

    let cycle = Duration::from_millis(10);
    let up = Duration::from_millis(20);
    let low = Duration::from_millis(2);
    let step = Duration::from_millis(2);

    motor.set_pin(21);
    board.set_shift_register_pins(16, 20, 19);

    board.set_motor(motor, n);
    board.test_motor_range(n, cycle, low, up, step);
}

fn start_motor_full(n: usize) {
    let mut board = AMSBoard::new(BoardType::BCM);
    let mut motor = Motor::new();

    motor.set_pin(21);
    board.set_shift_register_pins(16, 20, 19);

    board.set_motor(motor, n);
    board.start_motor_full(n);
    sleep(Duration::from_secs(2));
    board.stop_motor(n);

    board.invert_motor_direction(n);
    board.start_motor_full(n);
    sleep(Duration::from_secs(2));
    board.stop_motor(n);
}
