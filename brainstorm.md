What I want as a consumer of this lib:

- create an arduino motor shield
- add dc motors to the board, maximum of 4
    - should correspond with the output of that is printed on the board (M1-M4)
    - (?) add servo to the board
- start/stop dc motors
    - set pins of the motors
    - decide which motor output it will be on the ams (M1-M$)
    - use pwm to indicate the 'speed' of the motor
- (?) set servo to some degree


How do I want to do this as a user?

```
board <- create_board("BCM")
latch <- 1
clock <- 2
data <- 3
board.set_shift_register_pins(latch, clock, data)
motor1_pin <- 5
motor1 <- create_motor(pin)
board.add_motor(motor1, position=1)
board.motor(1).start()
sleep(100)
board.motor(1).stop()
```

Short description on how to achieve code with routines

```
create_board(board_type):
    set_pin_numbering(board_type)
```

```
set_shift_register_pins(latch, clock, data):
    set_latch(latch)
    set_clock(clock)
    set_data(data)
```

```
create_motor(pin):
    set_pin(pin)
    direction <- clockwise
    pwm <- 100
    is_running <- false
```

```
board::add_motor(&motor, position):
    self.set_motor(position, &motor)  
```

```
motor(position):
    return self.get_motor(position)
```

```
Motor {
    pin
    direction
    pwm
    is_running
}
```

```
Board {
    motors: [Option<Motor>; 4]
    register: Register
}
```

```
Register {
    latch_pin
    clock_pin
    data_pin
}
```
