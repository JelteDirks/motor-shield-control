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

