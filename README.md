# FLC16
An emulator for a console that does not exist.

Unlike PICO-8 or TIC-80, FLC16 does not interpret Lua code. It runs on it's own machine code compiled from it's own programming language, named Blackspace.  
It has a very limited set of features, with only 6 buttons (Arrow keys + Z and X), a 256x144 display, and 3 audio channels. These design choices were not born from laziness alone, but also an encouragement to challenge oneself to push their own limits. 

Here is a very useful [reference sheet](docs/refsheet.md) for developers when making their own FLC16 games.

### Downloads
Compiled copies of FLC16 can be found on the [Github Releases](https://github.com/FLC16/FLC16/releases)

You may also compile it yourself by cloning this repository, and running the following command:
```sh
$ cargo build --release
```

### Usage
Running the FLC16 executable without any parameters will show a "INSERT DISK" screen. Simply press "o" on your keyboard to open a file picker, and search for a `.flc` disk to load.  
You may also specify the file path as an argument, i.e. `flc16 example.flc`

FLC16 only has 6 buttons, being the arrow keys, Z, and X

To compile a blackspace file into a `.flc` disk, provide the `blackspace` argument followed by the file path. i.e. `flc16 blackspace example.bs`

### Tutorials
A list of tutorials can be found [here](docs/index.md)