use macroquad::{color::*, prelude::*};
use std::{collections::btree_map::Range, ops::RangeInclusive};

#[derive(Clone, PartialEq, Debug)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Clone)]
struct Board(Vec<Vec<(CellState)>>);

impl Board {
    /// Creates a new board from scratch. All the cells start dead by default.
    /// Output: A game of life board
    fn new(width: usize, length: usize) -> Self {
        let mut row = vec![(CellState::Dead); width];
        let mut board = vec![row.clone(); length];
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
        if self.0[y][x] == CellState::Alive {
            self.0[y][x] = CellState::Dead;
        } else {
            self.0[y][x] = CellState::Alive;
        }
    }

    /// Updates the states of every cell in the board
    fn update_board(&mut self) {
        //We need to store the old state so the updates on the cells don't cause confusion
        //For this, I clone the current board

        let board_old = &self.clone();

        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                self.update_cell_state(board_old, x, y);
            }
        }
    }

    fn update_cell_state(&mut self, old_board: &Board, x: usize, y: usize) {
        // Creates offset ranges for the neighbours, based on which offsets would be valid for the current position, so as to prevent overflow or underflow of indexes

        let x_offsets: std::ops::Range<isize> = if x > 0 && x < self.0[0].len() - 1 {
            -1..2
        } else if x > 1 {
            -1..1
        } else {
            0..2
        };
        let y_offsets: std::ops::Range<isize> = if y > 0 && y < self.0.len() - 1 {
            -1..2
        } else if y > 1 {
            -1..1
        } else {
            0..2
        };

        // Go through each neighbour and count the alive ones
        let mut alive_neighbours: u8 = 0;
        for x_offset in x_offsets {
            for y_offset in y_offsets.clone() {
                if (x_offset == 0 && y_offset == 0) {
                    continue;
                }
                //I use the overflowing adds so I can add a signed and unsigned integer, since I know oveflow/underflow aren't a risk
                if (old_board.0[y.overflowing_add_signed(y_offset).0]
                    [x.overflowing_add_signed(x_offset).0]
                    == CellState::Alive)
                {
                    alive_neighbours += 1;
                }
            }
        }

        //Change the cell state according to the number of neighbours
        match alive_neighbours {
            0..=1 => {
                self.0[y][x] = CellState::Dead;
            }
            3..=3 => {
                self.0[y][x] = CellState::Alive;
            }
            4..=8 => {
                self.0[y][x] = CellState::Dead;
            }
            _ => {}
        }
    }
}

fn is_input_numeric() -> bool {
    is_key_pressed(KeyCode::Key0)
        || is_key_pressed(KeyCode::Key1)
        || is_key_pressed(KeyCode::Key2)
        || is_key_pressed(KeyCode::Key3)
        || is_key_pressed(KeyCode::Key4)
        || is_key_pressed(KeyCode::Key5)
        || is_key_pressed(KeyCode::Key6)
        || is_key_pressed(KeyCode::Key7)
        || is_key_pressed(KeyCode::Key8)
        || is_key_pressed(KeyCode::Key9)
}

const DEFAULT_BOARD_LENGTH: usize = 30;
const DEFAULT_BOARD_WIDTH: usize = 50;

#[macroquad::main("Conway's Game of Life")]
async fn main() {

    // I set initial window width and height appropriately for the menu
    let mut window_width: f32 = screen_width();
    let mut window_height: f32 = window_width * 3. / 5.;

    request_new_screen_size(window_width, window_height);
    next_frame();

    //Variable that stores last time board was updated
    let mut last_update = get_time();

    let mut is_game_paused = true;

    //Used to time put the speed change message
    let mut last_text_update = get_time() - 1.;

    //Speeds. Each one contains the update interval for the board, and the speed to display when updated
    let speeds = [
        ("Speed: 0.25x", 2.),
        ("Speed: 0.5x", 1.),
        ("Speed: 1x", 0.5),
        ("Speed: 2x", 0.25),
        ("Speed: 4x", 0.125),
    ];
    let mut current_speed_index = 2;

    let mut board_width: usize = 0;
    let mut board_height: usize = 0;

    //Used to temporarily hold the width or height input by the user
    let mut current_size_input: usize = 0;

    //Used to konw whether width or height is selected in initial menu
    let mut currently_selected_width = true;

    while (!is_key_pressed(KeyCode::Enter)) {
        clear_background(LIGHTGRAY);

        let text_lines = 11.;

        draw_text(
            "GAME OF LIFE",
            window_width / 3.5,
            window_height / text_lines,
            60.,
            BLACK,
        );
        draw_text(
            "Rules:",
            window_width / 40.,
            2. * window_height / text_lines,
            30.,
            BLACK,
        );
        draw_text(
            "- Any alive cell with less than two neighbours dies by underpopulation",
            window_width / 40.,
            3. * window_height / text_lines,
            24.,
            BLACK,
        );
        draw_text(
            "- Any alive cell with more than three neighbours dies by overpopulation",
            window_width / 40.,
            4. * window_height / text_lines,
            24.,
            BLACK,
        );
        draw_text(
            "- Any dead cell with three neighbours becomes alive by reproduction",
            window_width / 40.,
            5. * window_height / text_lines,
            24.,
            BLACK,
        );
        draw_text(
            "Press space to pause. While paused, click on a cell to change its state",
            window_width / 27.,
            6. * window_height / text_lines,
            24.,
            BLACK,
        );
        draw_text(
            "While playing, press left or right to increase or decrease cell state update speed",
            window_width / 27.,
            7. * window_height / text_lines,
            20.,
            BLACK,
        );
        draw_text(
            "Press enter to start",
            window_width / 3.5,
            9.5 * window_height / text_lines,
            40.,
            BLACK,
        );

        if is_input_numeric() {
            current_size_input *= 10;
            let key = get_last_key_pressed().unwrap();
            match key {
                KeyCode::Key1 => current_size_input = current_size_input.saturating_add(1),
                KeyCode::Key2 => current_size_input = current_size_input.saturating_add(2),
                KeyCode::Key3 => current_size_input = current_size_input.saturating_add(3),
                KeyCode::Key4 => current_size_input = current_size_input.saturating_add(4),
                KeyCode::Key5 => current_size_input = current_size_input.saturating_add(5),
                KeyCode::Key6 => current_size_input = current_size_input.saturating_add(6),
                KeyCode::Key7 => current_size_input = current_size_input.saturating_add(7),
                KeyCode::Key8 => current_size_input = current_size_input.saturating_add(8),
                KeyCode::Key9 => current_size_input = current_size_input.saturating_add(9),
                _ => {}
            }
        }
        if is_key_pressed(KeyCode::Minus) {
            current_size_input /= 10;
        }

        //Swap around between selecting width or height to modify
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right) {
            currently_selected_width = !currently_selected_width;
            if currently_selected_width {
                current_size_input = board_width;
            } else {
                current_size_input = board_height;
            }
        }

        //Used to highlight the value currently being modified
        if currently_selected_width {
            board_width = current_size_input;
            draw_rectangle(
                2. * window_width / 6.,
                7.7 * window_height / text_lines,
                ((f64::log10((board_width + 1) as f64).floor() + 1.) * 11.) as f32,
                15.,
                YELLOW,
            );
        } else {
            board_height = current_size_input;
            draw_rectangle(
                4. * window_width / 6.,
                7.7 * window_height / text_lines,
                ((f64::log10((board_height + 1) as f64).floor() + 1.) * 11.) as f32,
                15.,
                YELLOW,
            );
        }

        draw_text(
            "Input board size: ",
            window_width / 30.,
            8. * window_height / text_lines,
            24.,
            BLACK,
        );
        draw_text(
            &board_width.to_string(),
            2. * window_width / 6.,
            8. * window_height / text_lines,
            24.,
            BLACK,
        );
        draw_text(
            " by ",
            3. * window_width / 6.,
            8. * window_height / text_lines,
            24.,
            BLACK,
        );
        draw_text(
            &board_height.to_string(),
            4. * window_width / 6.,
            8. * window_height / text_lines,
            24.,
            BLACK,
        );

        next_frame().await;
    }
    //I calculate the proportions of the board, to resize the window accordingly
    let board_proportions: f32 = board_width as f32 / board_height as f32;

    //If the board is wider than it is larger, I adapt the height. Else, I adapt the width accordingly
    //I do this since most screens are wider than they are long
    if board_proportions >= 1. {
        window_width = screen_width() * 4. / 5.;
        window_height = window_width / board_proportions;
    } else {
        window_height = screen_height();
        window_width = window_height * board_proportions;
    }
    let cell_size = window_width / (board_width as f32);
    //Since the OS bar on top of the window is counted for the height, I need to add a bit to it
    window_height += 0.08 * screen_height();;
    request_new_screen_size(window_width, window_height);
    next_frame();

    let mut game_board = Board::new(board_width, board_height);

    loop {
        let current_time = get_time();
      
        //If game is playing, I update the board in appropriate intervals. 
        //If not, I can swap the states of the cells by clicking on them
        if current_time >= (last_update + speeds[current_speed_index].1) && !is_game_paused {
            last_update = current_time;
            game_board.update_board();
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_position_x, mouse_position_y) = mouse_position();
            let cell_coordinate_x = (mouse_position_x / cell_size).floor() as usize;
            let cell_coordinate_y = (mouse_position_y / cell_size).floor() as usize;
            game_board.swap_cell_state(cell_coordinate_x, cell_coordinate_y);
        }

        //I draw each cell
        for y in 0..game_board.0.len() {
            for x in 0..game_board.0[y].len() {
                let x_screen_pos = (x as f32) * cell_size;
                let y_screen_pos = (y as f32) * cell_size;
                match game_board.0[y][x] {
                    CellState::Alive => {
                        draw_rectangle(x_screen_pos, y_screen_pos, cell_size, cell_size, BLACK);
                    }
                    CellState::Dead => {
                        draw_rectangle(x_screen_pos, y_screen_pos, cell_size, cell_size, WHITE);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            is_game_paused = !is_game_paused;
        }

        if is_game_paused {
            draw_text(
                "Paused",
                window_width / 80.,
                window_height / 15.,
                42.,
                BLACK,
            );
        } else {
            if is_key_pressed(KeyCode::Left) {
                if current_speed_index < 4 {
                    current_speed_index += 1;
                }
                last_text_update = get_time();
            }

            if is_key_pressed(KeyCode::Right) {
                if current_speed_index > 0 {
                    current_speed_index -= 1;
                }
                last_text_update = get_time();
            }
        }
        if get_time() < last_text_update + 0.75 {
            draw_text(
                speeds[current_speed_index].0,
                window_width / 80.,
                window_height / 15.,
                42.,
                BLACK,
            );
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_cell_with_two_alive_neighbours_stays_dead() {
        let mut board = Board::new(DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_LENGTH);
        board.swap_cell_state(0, 0);
        board.swap_cell_state(0, 1);

        board.update_board();
        assert_eq!(CellState::Dead, board.get_cell_status(1, 1));
    }

    #[test]
    fn dead_cell_with_three_alive_neighbours_revives() {
        let mut board = Board::new(DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_LENGTH);
        board.swap_cell_state(0, 0);
        board.swap_cell_state(0, 1);
        board.swap_cell_state(1, 0);

        board.update_board();
        assert_eq!(CellState::Alive, board.get_cell_status(1, 1));
    }

    #[test]
    fn alive_cell_with_two_alive_neighbours_stays_alive() {
        let mut board = Board::new(DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_LENGTH);
        board.swap_cell_state(0, 0);
        board.swap_cell_state(0, 1);
        board.swap_cell_state(1, 0);

        board.update_board();
        assert_eq!(CellState::Alive, board.get_cell_status(1, 0));
    }

    #[test]
    fn alive_cell_with_three_alive_neighbours_stays_alive() {
        let mut board = Board::new(DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_LENGTH);
        board.swap_cell_state(0, 0);
        board.swap_cell_state(0, 1);
        board.swap_cell_state(1, 0);
        board.swap_cell_state(1, 1);

        board.update_board();
        assert_eq!(CellState::Alive, board.get_cell_status(1, 0));
    }

    #[test]
    fn alive_cell_with_four_alive_neighbours_dies() {
        let mut board = Board::new(DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_LENGTH);
        board.swap_cell_state(1, 1);
        board.swap_cell_state(0, 1);
        board.swap_cell_state(0, 2);
        board.swap_cell_state(1, 0);
        board.swap_cell_state(2, 0);

        board.update_board();
        assert_eq!(CellState::Dead, board.get_cell_status(1, 1));
    }

    #[test]
    fn alive_cell_with_one_alive_neighbour_dies() {
        let mut board = Board::new(DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_LENGTH);
        board.swap_cell_state(1, 1);
        board.swap_cell_state(1, 0);

        board.update_board();
        assert_eq!(CellState::Dead, board.get_cell_status(1, 1));
    }
}
