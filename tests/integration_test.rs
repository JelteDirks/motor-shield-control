use motor_shield_control::board::{AMSBoard, BoardType};
use motor_shield_control::motor::{Motor, Direction};

#[test]
fn run_a_motor() {
    let mut board: AMSBoard = AMSBoard::new(BoardType::BCM);
    let mut m1: Motor = Motor::new();
    m1.set_direction(Direction::Counterclockwise);
    m1.set_pin(16);
    board.set_motor(m1, 1);
    assert_eq!(4, board.get_directions());
}
