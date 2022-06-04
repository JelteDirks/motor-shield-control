## The Motor Structure

The motor structure has the following properties:

#### pin

The GPIO pin where this motor will be addressed. This is the pin that is
connected to the enable input of the L293D of this specific motor. Make sure
that this pin is correctly set up according to the [the schematics](https://github.com/JelteDirks/motor-shield-control/blob/aab81d02e55ea8caf1bc7d661d84cc2a877116a5/arduino%20motor%20shield%20schematic.jpeg)
because you will have to assign it to the correct position on the board (M1-M4).


#### pwm

This is basically the adjustable power setting. 100 is the maximum, and entirely
off is the minimum. Anything in between will let you change the rotational
speed using pusle width modulation (PWM).

#### is_running()

This is an indication of the status of the motor. If is_running() returns true,
the motor is currently addressed by the data lines as either running clockwise
or counter clockwise.

#### direction

This is the direction that the motor is currently spinning in or will be
spinning once it is turned on.

#### set_pwm(int n)

This method sets the pwm value of this motor. ``` 0 < n <= 100```

