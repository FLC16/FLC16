# FLC16 reference sheet
as of 6/13/22

## Bits of Knowledge
I was not sure where to put this stuff as it didn't fit in any other category.  
FLC16 is an emulator for a console that does not exist. It uses a 256x144 pixel display with a 4 bit (16 color) palette. 
It has a virtual stack of 256 bytes, and a virtual memory size of 65536 bytes. 
The CPU architecture is completely unique and machine code must be compiled from Blackspace.
It has three sound channels, one triangle, one square, and one noise.

## CLI arguments
`flc16` - Start emulator with no disk loaded
`flc16 example.flc` - Start emulator with `example.flc` loaded
`flc16 blackspace code.bs` - Compile blackspace file `code.bs` into machine code

## Color index
FLC16 uses the [Go-Line Palette](https://lospec.com/palette-list/go-line)

* 0 Black #000000
* 1 Brown #ab5236
* 2 White #fff1e8
* 3 Orange #ff8426
* 4 Dark grey #5f574f
* 5 Yellow #ffdd34
* 6 Lime green #50e112
* 7 Sea green #3fa66f
* 8 Cyan #00ffcc
* 9 Light blue #29adff
* 10 Dark blue #365987
* 11 Blue #0033ff
* 12 Light grey #c2c3c7
* 13 Purple #430067
* 14 Magenta #430067
* 15 Red #ff004d

## Blackspace keywords
* `store` Pop three bytes (Value, X, and Y) and set the heap address 0xXY equal to Value
* `get` Pop two bytes (X and Y) and push the value at heap address 0xXY
* `print` Print the stack to console (useful for debugging)
* `push` Push the following byte(s) to the stack. e.g. `push 1; push 0xff 0xee;`
* `pop` Pop the last value from the stack
* `write` Pop three bytes (X, Y, and Color Index) and set the pixel at X and Y equal to Color Index
* `add` Pop A and B, push A+B (sums over 255 will wrap around to 0)
* `subtract` Pop A and B, push B-A (differences under 0 will wrap around to 255)
* `multiply` Pop A and B, push A\*B (products over 255 will wrap around to 0)
* `divide` Pop A and B, push B/A (rounded to the nearest whole number)
* `modulo` Pop A and B, push B%A
* `routine` Declare a routine at the address provided. e.g. `routine 0xff01`
* `end` End a routine
* `call` Pop X and Y, then call a routine at address 0xXY
* `duplicate` Duplicate the last value on the stack
* `swap` Pop A and B, push A and B
* `copy` Pop A, and push the value on the stack at index A
* `if` Call a routine if a condition is met. e.g. `if false 0x0001; if nonzero 0x0002`
    - The condition is popped from the top of the stack
    - If the condition is 0, it is `zero`. If it is any other number, it is `nonzero`
* `greaterthan` Pop A and B, push 1 if A\>B otherwise push 0
* `lessthan` Pop A and B, push 1 if A\<B otherwise push 0
* `sprite` Pop X and Y, and write the following bytes directly to video memory, line separated by 0xfe. e.g. `sprite 1 1 0xfe 2 2 0xfe 3 3`
* `bitnot` Pop A, push !A
* `bitand` Pop A and B, push A&B
* `bitor` Pop A and B, push A\|B
* `bitxor` Pop A and B, push A^B
* `random` Push a random byte from 0-255 to the stack
* `repeat` Pop N, and repeat the routine N number of times. e.g. `repeat 0x00ab`
* `beep` Pop Note and Length. Queue a Triangle sound to be played
	- Note is a number from 0 to 107 that represents a note from C0 to B8 respectively. C4 is at index 60.
	- Length is represented in deciseconds. (1/10 of a second)
* `boop` Pop Note and Length. Queue a Square sound to be played
* `noise` Pop only Length. Queue a Noise sound to be played
* `empty` Empty the sound queue and stop the player
* `alias` Create a textual alias for a u16 number. Reference it in `routine` or `push` with a `$` prefix e.g. `alias my_address 0x1234; push $my_address` 

## Reserved Addresses:
### Routines:
* `0xFFFF` Draw Event. Called on every frame
* `0xFFFE` Update. Called on every window update
* `0xFF01` Up arrow key pressed
* `0xFF02` Down arrow key pressed
* `0xFF03` Left arrow key pressed
* `0xFF04` Right arrow key pressed
* `0xFF05` Z pressed
* `0xFF06` X pressed
* `0xFF0F` Called every 1/X second (X is taken from heap `0xFF0F`)

### Heap addresses
* `0xFF01` Up arrow key held (boolean)
* `0xFF02` Down arrow key held (boolean)
* `0xFF03` Left arrow key held (boolean)
* `0xFF04` Right arrow key held (boolean)
* `0xFF05` Z held (boolean)
* `0xFF06` X held (boolean)
* `0xFF0F` Time interval to call `0xFF0F` routine (Integer) (Default 2)

Remember that the Heap and the Routine do NOT share the same address space.