## The Arduino Motor Shield Board Structure

#### dirSer

The serial / data line that provides data to the shift register.

#### dirClk

The shift register clock line which pushes the data line onto the shift register.

#### dirLat

The latch or register clock. This pushes the shift register data into memory
on the data lines.

#### motors

This holds the references to the motors. The index of the motor in the array is
the index of the motor on the board, with an increment. motors[0] => M1,
motors[1] => M2, etc.

#### get_motor(u8)

Returns the motor that belongs to that Motor slot. Beware that this is a 1 based
index, as the AMS has M1-M4.

#### set_motor(m &Motor, u8 n)

Sets the reference of a motor to the correct index on the board.

#### set_shift_register(ser u8, clk u8, lat u8)

Sets the shift register pin numbers of dirSer, dirClk and dirLat.

#### new(t BoardType)

Creates a new board instance of type t.


