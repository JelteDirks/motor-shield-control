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

These three inputs can also be found on the data lines of the shield itself, in
the upper left corner of the schematic. They are connected to the shift register and are used
to push the data into the memory. This happens as follows:

1) Set the DIR_LATCH to LOW, the connection to the memory register is now closed.
2) Set the DIR_SER to HIGH or LOW, depending on what you want to push
3) Set the DIR_CLK to HIGH, you have now pushed the bit into the first spot (QA)
4) repeat 2) and 3) until you have your required byte on the register.
5) Set the DIR_LATCH to HIGH, the stored byte is now put into the memory register
   and will be saved on the data lines

Note that you technically shift your most significant bit first (MSB). At least
in the representation that we advocating here, the most significant bit is
QH and the least significant bit is QA. You therefore need to start pushing in
from the MSB side of the byte.


Algorithm to push a byte onto the register (with comments):

```rust
latch.set_low(); // set the latch low before pushing
let mut b: u16 = 128; // initiate the MSB as the first bit to be pushed
while b != 0 { // continue as long as we still have to push a bit
    clock.set_low(); // set the clock low, we need a rising edge to push
    let c: u16 = b & (self.directions as u16); // bitwise AND with the bit to push
    if c == b { // if the directions had a 1 on that bit, it is still 1 and equal to the number we use to iterate
        serial.set_high(); // this means that the data line should be high for this push
    } else { // otherwise it was a 0
        serial.set_low(); // so we set the data line to low
    }
    clock.set_high(); // set the clock high, a rising edge will push the data line into the register
    b = b >> 1; // shift the bit to check to the right, the next lower significant bit
}        
latch.set_high(); // once all bits are pushed, store in memory by opening the latch
```

