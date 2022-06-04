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
board <- create_board()
latch <- 0
clock <- 0
data <- 0
board.set_shift_register_pins(latch, clock, data)
motor1 <- create_motor()
board.add_motor(motor1, position=1)
board.motor(1).start()
sleep(100)
board.motor(1).stop()
```
