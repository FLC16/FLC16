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