# Pixels
FLC16 has a display size of 256x144 pixels. This means 36864 total pixels for you to use. 

Each pixel can be one of 16 colors. Those colors are, in order,  
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

You can change the value of a pixel using the `write` command, which pops three numbers from the stack. The first is the X coordinate of the pixel, the second is the Y coordinate, and the third is the value (0-15) of the pixel. Easy.

```bs
# set the pixel at 5, 5 to Lime Green
push 6 5 5
write
```

If the routine at address `0xFFFF` is defined, then the display will be re-written every frame. Otherwise, pixels will stay on the display forever. 

```bs
# called every update and increases a variable by one each time
routine 0xfffe
    push 1 0 0
    get
    add
    push 0 0
    store
end

# called every frame, draws a lime pixel at X,5
routine 0xffff
    push 6 5 0 0
    get
    write
end
```

This is good for drawing individual pixels, but drawing a sprite with this method can be hard! That is why there is a `sprite` command that draws large numbers of pixels all at once. It pops 2 numbers from the stack, and uses them as the coordinate of the top-left corner of the sprite. You then provide the pixel values, and use `0xfe` to move down the Y axis.

```bs
# draw a checkered box at 5, 5
push 5 5
sprite 0 2 0 2 0xFE 2 0 2 0 0xFE 0 2 0 2 0xFE 2 0 2 0
```

You may need to draw lines at some point, and writing a line drawing algorithm by yourself is a huge pain! I've decided to give you a function that speeds up the process.
```bs
# color white
push 2

# first point, (0,0)
push 0 0

# second point, (5, 10)
push 5 10

line
```

You can also draw rectangles in the same manner as lines.
```bs
# color white
push 2

# first point, (3,7)
push 3 7

# width and height, (5, 10)
push 5 10

rectangle
```

Next lets look at [logic](07-logic.md)