# Game of Life
## An implementation in Rust of Conway's Game of Life

### What is it?

Conway's Game of Life is a deterministic simulation consisting of a grid of cells. Each cell can be alive or dead. Cells change state based on the following rules:
- A cell that's alive dies if it has less than two alive neighbours, as if by underpopulation
- A cell that's alive dies if it has more than three alive neighbours, as if by overpopulation 
- A cell that's dead becomes alive if it has exactly three neighbours, as if by reproduction
- A cell that's alive stays alive if it has two or three neighbours

### Project specifics

This project implements the game of life in Rust, using the Macroquad crate for GUI elements. You can input the desired board length and width on the starting menu; use the number keys to input a number, with minus as delete. You can move between width and height with the left and right keys.

Once you've input the board size, press enter to start. You can pause and unpause the game with spacebar. To swap the state of a cell, click on it with the game paused. You can also increase or decrease the board update speed with left and right.

### Dependencies 

- Rust 1.85.0
- Macroquad 0.4

### How to run

To build the project, run ```make``` on the command line. To run the tests, run ```make test```.
There is also a ```make clean``` target to delete the compiled binaries.
