# Implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust

RIP John Conway

## Usage
Start state is specified as a file (or optionally stdin). Whitespace is treated
as a dead cell and non-whitespace as a live cell. Newlines separate rows.

To specify a file, pass it as a command line parameter. To use stdin, either
pass `-` as the filename, or invoke the program with no arguments.

