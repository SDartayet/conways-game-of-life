use std::{collections::btree_map::Range, ops::RangeInclusive};

#[derive(Clone, PartialEq)]
enum CellState {
    Alive,
    Dead
}

#[derive(Clone)]
struct Board(Vec<Vec<(CellState)>>);

impl Board {

    /// Creates a new board from scratch. All the cells start dead by default.
    /// Output: A game of life board
    fn new() -> Self {
        let mut row = vec![(CellState::Dead); BOARD_WIDTH];
        let mut board = vec![row; BOARD_LENGTH];
        Board(board)
    }

    /// Swaps a specific position in an already existing board.
    /// Input: a mutable reference to the board, and the row and column of the cell to update
    /// NOT the cell udpate function, this one is intended to be used for the user to manually flip the states of cells before the game starts
    fn swap_cell_state(&mut self, x: usize, y: usize) {
        if self.0[col][row] == CellState::Alive { self.0[col][row] = CellState::Dead; } else { self.0[col][row] = CellState::Alive ;}
    }

    /// Updates the states of every cell in the board
    fn update_board (&mut self) {

        //We need to store the old state so the updates on the cells don't cause confusion
        //For this, I clone the current board

        let board_old = &self.clone();

        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                self.update_cell_state(board_old, x,y);
            }
        }
    }

    fn update_cell_state (&mut self, old_board:&Board, x: usize, y: usize) {

        // Creates offset ranges for the neighbours, based on which offsets would be valid for the current position, so as to prevent overflow or underflow of indexes

        let x_offsets: std::ops::Range<isize> = if x > 1 && x < BOARD_WIDTH-1 { -1..2 } 
        else if x > 1 { -1..1 } else { 0..2 };
        let y_offsets: std::ops::Range<isize> = if y > 1 && y < BOARD_WIDTH-1 { -1..2 } 
        else if y > 1 { -1..1 } else { 0..2 };
        
        // Go through each neighbour and count the alive ones
        let mut alive_neighbours:u8 = 0;
        for x_offset in x_offsets {
            for y_offset in y_offsets.clone() {
                if (x_offset == 0 && y_offset == 0) { continue; }
                //I use the overflowing adds so I can add a signed and unsigned integer, since I know oveflow/underflow aren't a risk
                if (old_board.0[y.overflowing_add_signed(y_offset).0][x.overflowing_add_signed(x_offset).0] == CellState::Alive ) { alive_neighbours += 1; }

                //Change the cell state according to the number of neighbours
                match alive_neighbours {
                    0..=1 => { self.0[y][x] = CellState::Dead; },
                    3..=3 => { self.0[y][x] = CellState::Alive; },
                    4..=8 => { self.0[y][x] = CellState::Dead; },
                    _ => {}
                }
            }
        }
    }   
}

const BOARD_LENGTH: usize = 10;
const BOARD_WIDTH: usize = 10;


fn main() {
    let mut game_board = Board::new();
    loop {
        game_board.update_board();
    }
}


