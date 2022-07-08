# Miscellaneous
You can use the `print` command to display the stack in the console, if there is a console. This does not pop any values.

You can use `duplicate` to duplicate the number at the top of the stack. This does not pop any values.

The `swap` command swaps the two highest numbers on the stack.

The `copy` command pops one number from the stack, and copies that index of the stack to the top
```bs
push 9 8 7 6 5 4
# Stack: [ 9, 8, 7, 6, 5, 4 ]

push 2
# The third index, since indexes start at 0
copy
# Stack: [ 9, 8, 7, 6, 5, 4, 7 ]
```

The bitwise operator `bitnot` pops one number, and flips all the bits in that number
```bs
push 85
bitnot
# Stack: [ 52 ]
```

The bitwise operators `bitand`, `bitor`, and `bitxor` each pop two values, A and B, and pushes the result of the corresponding bitwise function.
```bs
push 44 24
bitand
# Stack: [ 8 ]
```
```bs
push 44 24
bitor
# Stack: [ 60 ]
```
```bs
push 44 24
bitxor
# Stack: [ 52 ]
```

Random values can be produced by using the `random` command, which pushes a random number from 0-255 into the stack.

You can also repeat a routine N times by using the `repeat` command, which pops N from the stack.
```bs
routine 0xdddd
    # do something
end

push 5
repeat 0xdddd
# repeats the routine 5 times
```

Make your code look smaller by using semicolons `;` to separate commands
```bs
push 1 2; get; push 1; add; push 1 2; store
```

Keep track of your memory addresses and routines by using an `alias`
```bs
alias x_pos 0x0001
alias y_pos 0x0002
alias draw 0xffff
alias update 0xfffe

# use a dollar sign ($) to reference an alias

# set y pos to 5
push 5 $y_pos; store

routine $draw
    # aliases are always u16 numbers, meaning they take up 2 spaces in the stack
    push 2
    push $y_pos; get
    push $x_pos; get
    write
end

routine $update
    push $x_pos; get
    push 1; add
    push $x_pos; store
end
```

The color palette shipped with FLC16 is awesome, but some people may want to change it! Thankfully, I've added a `color` command to change the indexed colors  
It takes 4 arguments: Color index, R, G, and B
```bs
# change the 3rd color from white to pink (remember indexes start at 0, so the index will be 2)
color 2 0xd1 0xa0 0xf2
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