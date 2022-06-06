use motor_shield_control::board::{AMSBoard, BoardType};
use motor_shield_control::motor::{Motor, Direction, Status};
use std::matches;

#[test]
fn pre_setting_values() {
    let mut board: AMSBoard = AMSBoard::new(BoardType::BCM);
    let mut m1: Motor = Motor::new();
    m1.set_direction(Direction::Counterclockwise);
    board.set_motor(m1, 1);
    assert_eq!(8, board.get_directions());
}

#[test]
fn post_setting_values() {
    let mut board: AMSBoard = AMSBoard::new(BoardType::BCM);
    let mut m1: Motor = Motor::new();
    board.set_motor(m1, 1);
    board.change_motor_direction(1, Direction::Counterclockwise);
    assert_eq!(8, board.get_directions());

    let mut m2: Motor = Motor::new();
    board.set_motor(m2, 2);
    assert_eq!(10, board.get_directions());
}

#[test]
fn test_motor_changed_direction() {
    let mut board = AMSBoard::new(BoardType::BCM);
    let mut motor = Motor::new();
    board.set_motor(motor, 1);
    board.get_motor(1).as_mut().unwrap().set_direction(Direction::Counterclockwise);
    board.start_motor_full(1);
    assert!(matches!(board.get_motor(1).as_mut().unwrap().get_direction(), Direction::Counterclockwise));
}

#[test]
fn test_motor_default_direction() {
    let mut board = AMSBoard::new(BoardType::BCM);
    let mut motor = Motor::new();
    board.set_motor(motor, 1);
    board.start_motor_full(1);
    assert!(matches!(board.get_motor(1).as_mut().unwrap().get_direction(), Direction::Clockwise));
}

#[test]
fn test_motor_pin_setting() {
    let mut motor = Motor::new();
    motor.set_pin(16);

}

#[test]
fn test_motor_status() {
    let mut board = AMSBoard::new(BoardType::BCM);
    let mut motor = Motor::new();
    motor.set_pin(16);
    board.set_motor(motor, 1);
    board.start_motor_full(1);
    let status = board.get_motor(1).as_mut().unwrap().get_status();
    assert!(matches!(board.get_motor(1).as_mut().unwrap().get_status(), Status::Running));
}

#[test]
fn test_unset_motor() {
    let mut board = AMSBoard::new(BoardType::BCM);
    let result =  board.start_motor_full(1); 
    assert!(result.is_err());
}
