# Events and Input
Eventually you'll need to be able to accept input from the player.  
FLC16 has several reserved addresses in both routines and memory that will allow you to do this.  
Alongside input, there are other events that are provided for developers to utilize. 

First, the reserved memory addresses  
* `0xFF01` Up arrow key held (boolean)
* `0xFF02` Down arrow key held (boolean)
* `0xFF03` Left arrow key held (boolean)
* `0xFF04` Right arrow key held (boolean)
* `0xFF05` Z held (boolean)
* `0xFF06` X held (boolean)
* `0xFF0F` Time interval to call `0xFF0F` routine (Integer) (Default 2)

The addresses labeled as `boolean` will either be 1 or 0 depending on whether that key is being pressed. The other address is a little different, as you write to it instead of reading it. I will explain more in a bit.

Then, the reserved routine addresses  
* `0xFFFF` Draw Event. Called on every frame
* `0xFFFE` Update. Called on every window update
* `0xFF01` Up arrow key pressed
* `0xFF02` Down arrow key pressed
* `0xFF03` Left arrow key pressed
* `0xFF04` Right arrow key pressed
* `0xFF05` Z pressed
* `0xFF06` X pressed
* `0xFF0F` Called every 1/X second (X is taken from heap `0xFF0F`)

If a routine at any of these addresses has been declared, then it will be called by the emulator when the condition is met.  
`0xFF0F` is a routine that can be used as a timer every 1/X second. X is the number that you set in memory at address `0xFF0F` which defaults to 2. This mean it will be called every 1/2 second. 

```bs
# this routine will be called once every time Z is pressed
routine 0xff05
    # do something here
end

# this routine is called every time the window updates (a lot)
routine 0xfffe
    # check if Z is being held down
    push 0xff 0x05
    get
    # if Z is held, Stack: [ 1 ]
    # if not, Stack: [ 0 ]
end
```

The Draw Event routine is a little useless unless you know how to [write pixels to the display](06-pixels.md)