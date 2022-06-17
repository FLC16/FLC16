## Introducing Blackspace
Since FLC16 uses a kind of machine code that has never been seen before, I needed a way to assembly said code without wanting to tear my eyes out.

Blackspace is an assembly-like programming language created for FLC16 that was inspired by the [Whitespace](https://en.wikipedia.org/wiki/Whitespace_%28programming_language%29) esoteric language. Don't worry, this is much easier to read.

In FLC16, data is stored in something called a [Stack](https://en.wikipedia.org/wiki/Stack_%28abstract_data_type%29). Think of it like a stack of plates, where each plate is a single number. You can `push` numbers onto the top of the stack, and `pop` them off from the top. They cannot be removed from the middle or bottom. 

Below is an example of these functions in Blackspace. (Lines that start with `#` are comments!)
```bs
push 1
# Stack: [ 1 ]

push 5 8
# Stack: [ 1, 5, 8 ]

push 0x2c
# Stack: [ 1, 5, 8, 44 ]

pop
# Stack: [ 1, 5, 8 ]
```

The maximum size of the stack is 256 numbers.
Of course, this is not the only way to manipulate the stack. Let's move on to [mathematics](02-math.md)
