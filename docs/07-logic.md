# Logic
The `lessthan` and `greaterthan` commands are useful for checking if a number is less than another number, or greater than another number. Both commands pop two numbers from the stack, A and B, and push either 1 or 0. `lessthan` will push 1 if A < B, and `greaterthan` will push 1 if A > B. They both push 0 otherwise.

```bs
push 3 5
greaterthan
# Stack: [ 1 ]
# because 5 is greater than 3
```

```
push 3 5
lessthan
# Stack: [ 0 ]
# because 5 is NOT less than 3
```

You can use the `if` command to check if a value is zero or not, and then call a routine depending on the result.  
*this will pop a value from the stack!*

```bs
routine 0x1234
    # do something
end

push 3 5
lessthan
if zero 0x1234
# since 5 is not less than 3, the lessthan command pushed 0 to the stack
# that means 0x1234 WILL be called

push 3 5
greaterthan
if nonzero 0x1234
# since 5 is greater than 3, the greaterthan command pushed 1 to the stack
# that means 0x1234 WILL be called since 1 is nonzero
```

Next, lets look at producing [sound](08-sound.md)