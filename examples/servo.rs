use motor_shield_control::servo::*;

fn main() {
    let mut servo = Servo::new_default(21); // create a new servo with a pin number

    /// keep in mind that the servo works from a 0 point, which is all the way
    /// to one side, and can range from 0 to 180.
    ///
    /// this will set the angle to 90 degrees (so this is the middle or neutral
    /// position)
    servo.set_angle(90);
}
