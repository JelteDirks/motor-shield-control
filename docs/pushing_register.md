## Pushing Bytes onto the Shift Register

The shift register is a common entity in computer science, and the 74HC595 on 
the Arduino Motor Shield is no exception. I suggest you take a look at [the schematics](https://github.com/JelteDirks/motor-shield-control/blob/aab81d02e55ea8caf1bc7d661d84cc2a877116a5/arduino%20motor%20shield%20schematic.jpeg)
and look at the shift register layout. You will see three inputs that are of
importance in the setting of data lines:

| Input     | Use                                                                                                                                                                               |
|-----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| DIR_SER   | Also known as the data line, provides the data for pushing into the shift register                                                                                                |
| DIR_CLK   | Also known as the Shift Register Clock. On a rising edge of this input, the DIR_SER is pushed up into the shift register. Starting at QA, ending at QH                            |
| DIR_LATCH | Also konwn as the Register Clock. On a rising edge of this input, all parallel outputs of the register (QA-QH) are set according to byte that has been shifted into the register. |

These three inputs can also be found on the data lines of the shield itself in
the upper left corner. They are connected to the shift register and are used
to push the data into the memory. This happens as follows:

1) Set the DIR_LATCH to LOW, the connection to the memory register is now closed.
2) Set the DIR_SER to HIGH or LOW, depending on what you want to push
3) Set the DIR_CLK to HIGH, you have now pushed the bit into the first spot (QA)
4) repeat 2) and 3) until you have your required byte on the register.
5) Set the DIR_LATCH to HIGH, the stored byte is now put into the memory register
   and will be saved on the data lines

Note that you technically shift your least significant bit first (LSB). At least
in the representation that we advocating here, the least significant bit is
QH and the most significant bit is QA. You therefore need to start pushing in
from the LSB side of the byte.


Algorithm to push a byte onto the register:

```
PUSH_REG(pattern, data, clock, latch):
    GPIO.set(latch, LOW) // break connection with memory
    b <- 1 // start with LSB (bit pattern: 0b00000001)
    while b <= 0b10000000 // while b <= 128, stop after 7 bit shifts
        GPIO.set(clock, LOW) // start with a LOW clock
        c <- b & pattern // only keep bit b 
        if c == b // if c == b, bit b in pattern was 1 
            GPIO.set(data, HIGH) // we should push HIGH into the register
        else // c != b, bit b in pattern was 0
            GPIO.set(data, LOW) // we should push LOW into the register
        GPIO.set(clock, HIGH) // set clock to HIGH, rising edge pushes bit
        b <- b << 1 // shift b left once, testing the next significant bit
    GPIO.set(latch, HIGH) // save the byte into memory, put it on the data lines
```
