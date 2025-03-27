# Game of Life
## An implementation in Rust of Conway's Game of Life

### What is it?

Conway's Game of Life is a deterministic simulation consisting of a grid of cells. Each cell can be alive or dead. Cells change state based on the following rules:
- A cell that's alive dies if it has less than two alive neighbours, as if by underpopulation
- A cell that's alive dies if it has more than three alive neighbours, as if by overpopulation 
- A cell that's dead becomes alive if it has exactly three neighbours, as if by reproduction
- A cell that's alive stays alive if it has two or three neighbours

### Project specifics
This project implements the game of life in Rust, using the Macroquad crate for GUI elements

### Dependencies 
- Rust 1.85.0
- Macroquad 0.4

### How to run
To build the project, run
'''
cargo build
'''
on the command line.
