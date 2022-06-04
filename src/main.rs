mod motor;

fn main() {
    println!("deeznuts");
    let motor = motor::Motor::new();
    let running = motor.is_running();
    println!("running {:?}", running)
}

