# Sound
FLC16 has three different sound channels that are available to use. One triangle wave, one square wave, and one noise wave. 

For triangle waves, you can use the `beep` command. This pops 2 numbers from the stack, the first being a number (0-107) that represents a note from C0 to B8, and the second is the duration that this sound will play for, in deciseconds. (1/10 of a second)  
Middle C would be the number 60

```bs
# play a triangle wave for 3 seconds on A4
push 30 69
beep
```

The `boop` command for square waves is identical to `beep`, except these can be played at the same time. Two square notes/two triangle notes cannot be played at the same time, and are added to a queue instead. 

The `noise` command only pops one number, being the duration to play in deciseconds. A note is not required. This also has a queue, and can be played alongside square and triangle. 

You can then use the `empty` command to clear all the sound queues and stop all the playing sounds.

Last tutorial for [miscellaneous](09-misc.md) examples