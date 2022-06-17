# FLC16
An emulator for a console that does not exist.

Unlike PICO-8 or TIC-80, FLC16 does not interpret Lua code. It uses it's own machine code that is compiled from the Blackspace programming language.  
It has a very limited set of features, with only 6 buttons (Arrow keys + Z and X), a 256x144 display, and 6 audio channels. These design choices were not born from laziness alone, but also an encouragement to challenge oneself to push their own limits. 

Here is a very useful [reference sheet](refsheet.md) for developers when making their own FLC16 games.

### Downloads
Compiled copies of FLC16 can be found on the [Github Releases](https://github.com/FLC16/FLC16/releases)

You may also compile it yourself by cloning this repository, and running the following command:
```sh
$ cargo build --release
```

### Tutorials
(wip)
