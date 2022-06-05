use motor_shield_control::board::{AMSBoard, BoardType};
use motor_shield_control::motor::{Motor, Direction};

#[test]
fn pre_setting_values() {
    let mut board: AMSBoard = AMSBoard::new(BoardType::BCM);
    let mut m1: Motor = Motor::new();
    m1.set_direction(Direction::Counterclockwise);
    m1.set_pin(16);
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
