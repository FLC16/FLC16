# Random Access
Holding values in the stack can be a bit of a struggle. Thankfully, we don't need to. Alongside the stack, FLC16 has virtual [RAM](https://en.wikipedia.org/wiki/Random-access_memory) with a size of 65536 bytes.  

Each "address" in RAM can hold a single number, like a key-value database. For example, I can set the memory cell at address 0x1234 equal to 3, and then later retrieve the value at address 0x1234 which gives me the number 3. This is done by using the `store` and `get` commands. 

`store` pops 3 numbers from the stack. The first two numbers are used as the address (pop XX and YY, then the address is 0xXXYY), and the third number is the number to be stored at that address.  
`get` pops 2 numbers from the stack. Those will be the address to retrieve from. Then, it pushes the number that it found at that address into the stack.

```bs
push 5
# this is a number we need to store
# Stack: [ 5 ]

push 0x00 0x01
# we will use 0x0001 as the address in memory
# Stack: [ 5, 0, 1 ]

store
# Stack: []

# then, later in the script, we need to get our stored number
push 0x00 0x01
# the address did not change. it is still 0x0001
# Stack: [ 0, 1 ]

get
# Stack: [ 5 ]
```

I don't have a good transition into the next tutorial. [Routines](04-routines.md)