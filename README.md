
# Arduino Motor Shield

A Rust library for controlling DC or stepper motors with the Arduino Motor Control Shield for (2x) L293D.

---

This library is written as a means to provide Rust support for the Arduino
Motor Shield and operating 4 motors and 2 servos. To use this library, some
schematics are provided in the [schematics](https://github.com/JelteDirks/motor-shield-control/tree/main/schematics)
directory. To start off using this library, make sure that you take a look at
the schematics and feel confident that you at least have some knowledge about:

- basic electronics
- bitwise operations
- rust
- shift registers

### Shift Register

The shift register has its own topic in the docs files, but I will introduce
it here shortly. The shift enables one to use only 3 data lines to store a full
byte into memory. This is accomplished by sequentially pushying pushing data
into a register, moving the bits to the next data line. Once you have the byte
you want, you open a latch and the byte is stored into a memory register which
will retain these values untill power is withdrawn.

### DC Motors

There is great documentation to be found on the workings of DC motors, so all
that is important here is that we use the L293D register to power a maximum of
4 DC motors. The data lines from the shift register are fed into the inputs of
these two L293D that are present on the arduino motor shield. More on this in
the [docs of the motor directions](https://github.com/JelteDirks/motor-shield-control/blob/main/docs/motor_directions.md)
and on the [schematics](https://github.com/JelteDirks/motor-shield-control/tree/main/schematics).

### L293D

Documentation about the L293D is also in abundance, so we will go over the
important parts again. To make the DC motors change direction, the L293D makes
use of an H bridge. It will let current flow to the DC motors in a controlled
manner, and handle any counter-electromotive force that the DC motors will
generate once they are stopped abruptly.

### Pulse Width Modulation (PWM)

Pulse width modulation is the repeatedly setting of a data line to high and
low, which gives us an easy but very fine grained way of modulating the voltage
on a line without using a lot of power with variable voltage power supplies
and only using binary data outputs (0V and 5V). PWM is used to give the DC
motors a certain speed, and give the servo motor a certain direction it should
go to.

### Servo

A servo is a motor that is geared to go into a certain position once it
receives a pulse width modulation setting. In order to prevent writing the
entire internets documentation in this README, once again the important bits:
by addressing the servo enable line with a PWM signal, the tachometer inside
the servo motor will give feedback on it's current state and compare this to
the received input, after which it will try to stabilize by moving the motor
in a certain direction. Once it is stabilized, it will no longer try to move.
This enables us to give a certain PWM signal that will always move the servo
arm into a fixed position.
