# Mathematics
Computers wouldn't be very useful if not for their ability to manipulate numbers. FLC16 has 5 different mathematical commands for you to use: `add`, `subtract`, `multiply`, `divide`, and `modulo`

Using these commands does not require any arguments, since they will take numbers from the stack!  
Take this example Blackspace code below. Remember that lines beginning with `#` are comments, not code.
```bs
# push 1 and 2 into the stack
push 1 2

# add them together. this will remove them both from the stack, and replace them with their sum
add

# The stack should now contain [ 3 ]
```
It should also be noted that the highest value of a number you can use is 255

But be careful, the order of numbers on the stack is important. 
```bs
push 1 2
subtract
# Pop A and B, (2 and 1 respectively) and push B - A (1 - 2)
# Normally, this would be -1, but FLC16 doesn't support negative numbers.
# Instead, it wraps around to the highest possible value (255)

push 2 1
subtract
# Result: [ 1 ]
```
Any numbers below 0 will be wrapped around to 255. Any values above 255 will be wrapped around to 0. Simple as.

```bs
push 5 4
multiply
# Result: [ 20 ]
```

Since FLC16 does not support floating point numbers, division is rounded to the nearest integer. 
```bs
push 20 4
divide
# Result: [ 5 ]

push 20 4
divide
# Result: [ 5 ]

push 15 2
divide
# Result: [ 7 ]
```

The modulo operator returns the remainder in integer division.
```bs
push 15 2
modulo
# Result: [ 1 ]
```

But what good does this do if we don't have anywhere to store these numbers? Let's learn about [memory](03-mem.md)