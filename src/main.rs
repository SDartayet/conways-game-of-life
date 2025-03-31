use std::{collections::btree_map::Range, ops::RangeInclusive};
use macroquad::{ color::*, miniquad::window, prelude::* };

#[derive(Clone, PartialEq, Debug)]
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
        let mut row = vec![(CellState::Dead); DEFAULT_BOARD_WIDTH];
        let mut board = vec![row.clone(); DEFAULT_BOARD_LENGTH];
        Board(board)
    }

    /// Returns the status of a cell, given its coordinates
    /// Input: x and y coordinates of the cell
    /// Output: whether the cell is dead or alive
    fn get_cell_status(&self, x: usize, y: usize) -> CellState {
        self.0[y][x].clone()
    }

    /// Swaps a specific position in an already existing board.
    /// Input: a mutable reference to the board, and the row and column of the cell to update
    /// NOT the cell udpate function, this one is intended to be used for the user to manually flip the states of cells before the game starts
    fn swap_cell_state(&mut self, x: usize, y: usize) {
        if self.0[y][x] == CellState::Alive { self.0[y][x] = CellState::Dead; } else { self.0[y][x] = CellState::Alive ;}
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

        let x_offsets: std::ops::Range<isize> = if x > 0 && x < DEFAULT_BOARD_WIDTH-1 { -1..2 } 
        else if x > 1 { -1..1 } else { 0..2 };
        let y_offsets: std::ops::Range<isize> = if y > 0 && y < DEFAULT_BOARD_LENGTH-1 { -1..2 } 
        else if y > 1 { -1..1 } else { 0..2 };
        
        // Go through each neighbour and count the alive ones
        let mut alive_neighbours:u8 = 0;
        for x_offset in x_offsets {
            for y_offset in y_offsets.clone() {
                if (x_offset == 0 && y_offset == 0) { continue; }
                //I use the overflowing adds so I can add a signed and unsigned integer, since I know oveflow/underflow aren't a risk
                if (old_board.0[y.overflowing_add_signed(y_offset).0][x.overflowing_add_signed(x_offset).0] == CellState::Alive ) { alive_neighbours += 1; }
            }
        }

        //Change the cell state according to the number of neighbours
        match alive_neighbours {
            0..=1 => { self.0[y][x] = CellState::Dead; },
            3..=3 => { self.0[y][x] = CellState::Alive; },
            4..=8 => { self.0[y][x] = CellState::Dead; },
            _ => {}
        }
    }   
}

const DEFAULT_BOARD_LENGTH: usize = 30;
const DEFAULT_BOARD_WIDTH: usize = 50;

#[macroquad::main("Conway's Game of Life")]
async fn main() {

    let board_proportions: f32 = DEFAULT_BOARD_WIDTH as f32 / DEFAULT_BOARD_LENGTH as f32;
    let mut window_width = screen_height() * board_proportions;
    let mut window_height: f32;
    if window_width > screen_width() {
        window_height = screen_width() / board_proportions;
        window_width = screen_width(); 
    } else {
        window_height = screen_height();
    }
    let cell_size = window_width / DEFAULT_BOARD_WIDTH as f32;
    clear_background(WHITE);

    request_new_screen_size(window_width, window_height);
    next_frame();
    let mut last_update = get_time();

    let mut game_board = Board::new();

    let mut is_game_paused = true;

    let speeds = 

    

    game_board.swap_cell_state(20, 20);
    game_board.swap_cell_state(20, 21);
    game_board.swap_cell_state(21, 20);
    game_board.swap_cell_state(20, 19);
    game_board.swap_cell_state(19, 20);
    

    while (!is_key_pressed(KeyCode::Enter)) {
        clear_background(LIGHTGRAY);
        //draw_rectangle(0., window_height / 4., window_width, window_height / 2., YELLOW);
        draw_text("GAME OF LIFE", window_width / 3.5, window_height / 9., 60., BLACK);
        draw_text("Rules:", window_width / 40., 2.* window_height / 9., 30., BLACK);
        draw_text("- Any alive cell with less than two neighbours dies by underpopulation", window_width / 40., 3.* window_height / 9., 24., BLACK);
        draw_text("- Any alive cell with more than three neighbours dies by overpopulation", window_width / 40., 4.* window_height / 9., 24., BLACK);
        draw_text("- Any dead cell with three neighbours becomes alive by reproduction", window_width / 40., 5.* window_height / 9., 24., BLACK);
        draw_text("Press space to pause. While paused, click on a cell to change its state", window_width / 27., 6.* window_height / 9., 24., BLACK);
        draw_text("Press enter to start", window_width / 3.5, 7.* window_height / 9., 40., BLACK);


        next_frame().await;
    }

    loop {
        let current_time = get_time();

        let cell_size = window_width / DEFAULT_BOARD_WIDTH as f32;
        
        if current_time >= (last_update + 0.5) && !is_game_paused {
            last_update = current_time;
            game_board.update_board();
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_position_x, mouse_position_y) = mouse_position();
            let cell_coordinate_x = (mouse_position_x / cell_size).floor() as usize;
            let cell_coordinate_y = (mouse_position_y / cell_size).floor() as usize;
            game_board.swap_cell_state(cell_coordinate_x, cell_coordinate_y);

        }

        for y in 0..game_board.0.len() {
            for x in 0..game_board.0[y].len() {
                let x_screen_pos = (x as f32) * cell_size;
                let y_screen_pos = (y as f32) * cell_size;
                match game_board.0[y][x] {
                    CellState::Alive => { draw_rectangle(x_screen_pos, y_screen_pos, cell_size, cell_size, BLACK); },
                    CellState::Dead => { draw_rectangle(x_screen_pos, y_screen_pos, cell_size, cell_size, WHITE); }
                }
            }
        }

        if is_key_pressed(KeyCode::Space) { is_game_paused = !is_game_paused; }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_cell_with_two_alive_neighbours_stays_dead() {
        let mut board = Board::new();
        board.swap_cell_state(0,0);
        board.swap_cell_state(0,1);

        board.update_board();
        assert_eq!(CellState::Dead, board.get_cell_status(1, 1));
    }

    #[test]
    fn dead_cell_with_three_alive_neighbours_revives() {
        let mut board = Board::new();
        board.swap_cell_state(0,0);
        board.swap_cell_state(0,1);
        board.swap_cell_state(1, 0);

        board.update_board();
        assert_eq!(CellState::Alive, board.get_cell_status(1, 1));
    }

    #[test]
    fn alive_cell_with_two_alive_neighbours_stays_alive() {
        let mut board = Board::new();
        board.swap_cell_state(0,0);
        board.swap_cell_state(0,1);
        board.swap_cell_state(1,0);

        board.update_board();
        assert_eq!(CellState::Alive, board.get_cell_status(1, 0));
    }

    #[test]
    fn alive_cell_with_three_alive_neighbours_stays_alive() {
        let mut board = Board::new();
        board.swap_cell_state(0,0);
        board.swap_cell_state(0,1);
        board.swap_cell_state(1,0);
        board.swap_cell_state(1,1);

        board.update_board();
        assert_eq!(CellState::Alive, board.get_cell_status(1, 0));
    }

    #[test]
    fn alive_cell_with_four_alive_neighbours_dies() {
        let mut board = Board::new();
        board.swap_cell_state(1,1);
        board.swap_cell_state(0,1);
        board.swap_cell_state(0,2);
        board.swap_cell_state(1,0);
        board.swap_cell_state(2,0);

        board.update_board();
        assert_eq!(CellState::Dead, board.get_cell_status(1, 1));
    }

    #[test]
    fn alive_cell_with_one_alive_neighbour_dies() {
        let mut board = Board::new();
        board.swap_cell_state(1,1);
        board.swap_cell_state(1,0);

        board.update_board();
        assert_eq!(CellState::Dead, board.get_cell_status(1, 1));
    }
    
}


