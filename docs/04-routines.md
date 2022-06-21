# Routines
At times you'll need to run a series of commands multiple times, or you just want to organize them. In most languages, you'd use a function for this. However, a function accepts parameters and returns a value. Routines do not. They are simply a series of instructions that can be replayed. 

Similar to memory, each routine has an address. Memory and Routines do **NOT** share the same address space. 0x1234 in memory is completely different from 0x1234 in the routine list.  
Since addresses is 16 bit, this means there can be a maximum of 65536 routines.

Declaring a routine is as follows:
```bs
# routine that gets a number from memory and increases it
routine 0x0001
    push 1 1
    get
    push 1
    add
    push 1 1
    store
end
```
Opening with the keyword `routine` followed by the address, and closing with the keyword `end`

Routines can be declared inside other routines, but they will share the same `end` 

They can then be called/executed by using the `call` command. This command pops 2 numbers from stack, which are used as the address to execute. (pop XX and YY, call 0xXXYY)

```bs
routine 0x0001
    # etc
end

push 0 1
call
```

Be mindful that there are certain addresses in both routines and memory that are reserved for [events and input](05-reserved.md)