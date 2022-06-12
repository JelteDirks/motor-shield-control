## Calculating Shift Register Byte for Directions

Because the shift register of the arduino motor shield decides which direction
all four motors go. We have to shift the correct byte into the shift register
before latching it to the data lines. If you continue reading this document,
I suggest you have [the schematics](https://github.com/JelteDirks/motor-shield-control/blob/aab81d02e55ea8caf1bc7d661d84cc2a877116a5/arduino%20motor%20shield%20schematic.jpeg)
open as it might help you understand it better.

The directions of the motor spin is decided by the direction of the current
through the L293D inputs. If one is low, and one is high, the motor will be
spinning in some direction. If the vice-versa situation is set, the motor will
be spinning in the other direction. If the L293D inputs are the same, the 
motors will stall.

The L293D also has to be enabled using the enable inputs. If the enable inputs
are low, there is no current flowing to the motor. This is also the input for
which we can use PWM.

To make it a little easier for the consumer to work with the library, we use
the indicated positions on the arduino motor shield as an index for the motors
that the consumer will use. As an example:

```
board.set_motor(&motor, 1) // set the motor of position 1 (M1)
board.start_motor_full(1) // starts the motor at M1 at full speed
```

To guide the consumer of the library a little better, the following table
should give some guidance on which direction the motors spin in with which
configuration of the shift register.

| Motor | Clockwise | Counterclockwise |
|-------|-----------|------------------|
| M1    | 4         | 8                |
| M2    | 2         | 16               |
| M3    | 1         | 64               |
| M4    | 32        | 128              |

To see why this is the case, we show the table with bit representation for each
data line of the shift register, which correspond with the number in the table
above (C=clockwise and CC=counter clockwise):

| Motor | Direction | M3A(QA) | M2A(QB) | M1A(QC) | M1B(QD) | M2B(QE) | M4A(QF) | M3B(QG) | M4B(QH) |
|-------|-----------|---------|---------|---------|---------|---------|---------|---------|---------|
| M1    | C         | 0       | 0       | 1       | 0       | 0       | 0       | 0       | 0       |
| M1    | CC        | 0       | 0       | 0       | 1       | 0       | 0       | 0       | 0       |
| M2    | C         | 0       | 1       | 0       | 0       | 0       | 0       | 0       | 0       |
| M2    | CC        | 0       | 0       | 0       | 0       | 1       | 0       | 0       | 0       |
| M3    | C         | 1       | 0       | 0       | 0       | 0       | 0       | 0       | 0       |
| M3    | CC        | 0       | 0       | 0       | 0       | 0       | 0       | 1       | 0       |
| M4    | C         | 0       | 0       | 0       | 0       | 0       | 1       | 0       | 0       |
| M4    | CC        | 0       | 0       | 0       | 0       | 0       | 0       | 0       | 1       |

Now as stated earlier, the motors do not move if both inputs are high or low
simultaneously. This means that to set certain directions, we can not just
shift the number for the corresponding motor with direction onto the shift
register, since the other motors will stall. Using the bitwise OR operation
on the numbers, we can set multiple motors to go in some direction. An example:

I want to set motors M1 and M2 to clockwise, and motor M3 to counter clockwise.
To achieve this, we take the number for the motor and direction, and we bitwise
OR these with eachother. In code we can use the decimal notation to make it 
easier, but for illustration purposes we will demonstrate with the binary 
representation:

|       | Direction | M3A(QA) | M2A(QB) | M1A(QC) | M1B(QD) | M2B(QE) | M4A(QF) | M3B(QG) | M4B(QH) |
|-------|-----------|---------|---------|---------|---------|---------|---------|---------|---------|
| M1    | C         | 0       | 0       | 1       | 0       | 0       | 0       | 0       | 0       |
| M2    | C         | 0       | 1       | 0       | 0       | 0       | 0       | 0       | 0       |
| M3    | CC        | 0       | 0       | 0       | 0       | 0       | 0       | 1       | 0       |
| Final |           | 0       | 1       | 1       | 0       | 0       | 0       | 1       | 0       |

Our final number is 01100010 or in decimal: 98. Shifting the number 98 onto the
shift register will set the above configuration. **Important**: M4 will stall after
pushing this to the register, both inputs are 0!

Below is an algorithm provided for calculating the byte that has to be put into
the memory of the shift register. It is important to keep in mind that this 
calculation has to incorporate the settings for each motor.

```rust
fn calculate_directions(&self) -> u8 {
    let m1_dir: u8 = match &self.motors[0] {
        Some(m) => match m.get_direction(){
            Direction::Clockwise => 4,
            Direction::Counterclockwise => 8
        },
        _ => 0
    };
    let m2_dir: u8 = match &self.motors[1] {
        Some(m) => match m.get_direction(){
            Direction::Clockwise => 2,
            Direction::Counterclockwise => 16 
        },
        _ => 0
    };
    let m3_dir: u8 = match &self.motors[2] {
        Some(m) => match m.get_direction(){
            Direction::Clockwise => 1,
            Direction::Counterclockwise => 64 
        },
        _ => 0
    };
    let m4_dir: u8 = match &self.motors[3] {
        Some(m) => match m.get_direction(){
            Direction::Clockwise => 32,
            Direction::Counterclockwise => 128 
        },
        _ => 0
    };
    return m1_dir | m2_dir | m3_dir | m4_dir; 
}
```
To explain the above algorithm: its only function is to retrieve the direction
of every motor, of which the corresponding numbers are fixed, and bitwise OR
or these with eachother. Since the bit representation of these numbers never
have an overflow, we could also add the numbers together, yet this better
represents what is happening and why.

When the directions are calculated, we have to push these into the shift
register and save them to the memory so the motors have a fixed direction. To
do this, we use the following algorithm:





