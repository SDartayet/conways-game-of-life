use std::ops::{Index, IndexMut, RangeInclusive};

use macroquad::{color::*, prelude::*};

#[derive(Clone, Copy, PartialEq, Debug)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Clone)]
struct Board {
    board: Vec<CellState>,
    old_board: Vec<CellState>,
    width: usize,
    length: usize,
}

impl Board {
    /// Creates a new board from scratch. All the cells start dead by default.
    /// Output: A game of life board
    fn new(width: usize, length: usize) -> Self {
        let board = vec![CellState::Dead; width * length];
        let old_board = board.clone();
        Board {
            board,
            old_board,
            width,
            length,
        }
    }

    /// Swaps a specific position in an already existing board.
    /// Input: a mutable reference to the board, and the row and column of the cell to update
    /// NOT the cell udpate function, this one is intended to be used for the user to manually flip the states of cells before the game starts
    fn toggle_cell_state(&mut self, x: usize, y: usize) {
        match self[(x, y)] {
            CellState::Alive => self[(x, y)] = CellState::Dead,
            CellState::Dead => self[(x, y)] = CellState::Alive,
        }
    }

    /// Updates the states of every cell in the board
    fn update_board(&mut self) {
        self.old_board = self.board.clone();
        for x in 0..self.width {
            for y in 0..self.length {
                self.update_cell_state(x, y);
            }
        }
    }

    fn update_cell_state(&mut self, x: usize, y: usize) {
        // Creates offset ranges for the neighbours, based on which offsets would be valid for the current position, so as to prevent overflow or underflow of indexes
        let x_neighbours =
            RangeInclusive::new(x.checked_sub(1).unwrap_or(0), (self.width - 1).min(x + 1));
        let y_neighbours =
            RangeInclusive::new(y.checked_sub(1).unwrap_or(0), (self.length - 1).min(y + 1));

        // Go through each neighbour and count the alive ones
        let mut alive_neighbours: u8 = 0;
        for x_neighbour in x_neighbours {
            for y_neighbour in y_neighbours.clone() {
                if (x_neighbour, y_neighbour) == (x, y) {
                    continue;
                }
                if self.old_board[x_neighbour * self.width + y_neighbour] == CellState::Alive {
                    alive_neighbours += 1;
                }
            }
        }

        //Change the cell state according to the number of neighbours
        match alive_neighbours {
            0..=1 => {
                self[(x, y)] = CellState::Dead;
            }
            3 => {
                self[(x, y)] = CellState::Alive;
            }
            4.. => {
                self[(x, y)] = CellState::Dead;
            }
            _ => {}
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = CellState;

    /// Returns the status of a cell, given its coordinates
    /// Input: x and y coordinates of the cell
    /// Output: whether the cell is dead or alive
    fn index(&self, (x, y): (usize, usize)) -> &CellState {
        &self.board[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Board {
    /// Returns a mutable reference to the status of a cell, given its coordinates
    /// Input: x and y coordinates of the cell
    /// Output: a mutable reference to the cell state
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut CellState {
        &mut self.board[y * self.width + x]
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
        ("Speed: 0.25x", 0.125),
        ("Speed: 0.5x", 0.25),
        ("Speed: 1x", 0.5),
        ("Speed: 2x", 1.),
        ("Speed: 4x", 2.),
    ];
    let mut current_speed_index = 2;

    let mut board_width: usize = 10;
    let mut board_height: usize = 10;

    //Used to temporarily hold the width or height input by the user
    let mut current_size_input: usize = 10;

    //Used to konw whether width or height is selected in initial menu
    let mut currently_selected_width = true;

    while !is_key_pressed(KeyCode::Enter) {
        clear_background(LIGHTGRAY);

        let text_lines = 12.;

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
            10.5 * window_height / text_lines,
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
        draw_text(
            "(use left and right to swap between width and height, input a number with numkeys, delete with minus)",
            window_width / 200.,
            9. * window_height / text_lines,
            18.,
            BLACK,
        );

        next_frame().await;
    }
    //I calculate the proportions of the board, to resize the window accordingly
    let board_proportions = board_width / board_height;

    //I adapt the screen size to the board proportions, according to whether the screen is longer than it's tall, or viceversa
    //I need to multiply or divide by the proportions so cells are always square shaped
    if screen_height() < screen_width() {
        window_width = screen_height() * board_proportions as f32;
    } else {
        window_height = screen_width() / board_proportions as f32;
    }
    let cell_size = window_width / (board_width as f32);
    //Since the OS bar on top of the window is counted for the height, I need to add a bit to it
    //window_height += 0.08 * screen_height();
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
        }
        if is_game_paused && is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_position_x, mouse_position_y) = mouse_position();
            let cell_coordinate_x = (mouse_position_x / cell_size).floor() as usize;
            let cell_coordinate_y = (mouse_position_y / cell_size).floor() as usize;
            game_board.toggle_cell_state(cell_coordinate_x, cell_coordinate_y);
        }

        //I draw each cell
        for x in 0..game_board.width {
            for y in 0..game_board.length {
                let x_screen_pos = (x as f32) * cell_size;
                let y_screen_pos = (y as f32) * cell_size;
                match game_board[(x, y)] {
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
                LIGHTGRAY,
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
                LIGHTGRAY,
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
        let mut board = Board::new(3, 3);
        board.toggle_cell_state(0, 0);
        board.toggle_cell_state(0, 1);

        board.update_board();
        assert_eq!(CellState::Dead, board[(1, 1)]);
    }

    #[test]
    fn dead_cell_with_three_alive_neighbours_revives() {
        let mut board = Board::new(3, 3);
        board.toggle_cell_state(0, 0);
        board.toggle_cell_state(0, 1);
        board.toggle_cell_state(1, 0);

        board.update_board();
        assert_eq!(CellState::Alive, board[(1, 1)]);
    }

    #[test]
    fn alive_cell_with_two_alive_neighbours_stays_alive() {
        let mut board = Board::new(3, 3);
        board.toggle_cell_state(0, 0);
        board.toggle_cell_state(0, 1);
        board.toggle_cell_state(1, 0);

        board.update_board();
        assert_eq!(CellState::Alive, board[(1, 0)]);
    }

    #[test]
    fn alive_cell_with_three_alive_neighbours_stays_alive() {
        let mut board = Board::new(3, 3);
        board.toggle_cell_state(0, 0);
        board.toggle_cell_state(0, 1);
        board.toggle_cell_state(1, 0);
        board.toggle_cell_state(1, 1);

        board.update_board();
        assert_eq!(CellState::Alive, board[(1, 0)]);
    }

    #[test]
    fn alive_cell_with_four_alive_neighbours_dies() {
        let mut board = Board::new(4, 4);
        board.toggle_cell_state(1, 1);
        board.toggle_cell_state(0, 1);
        board.toggle_cell_state(0, 2);
        board.toggle_cell_state(1, 0);
        board.toggle_cell_state(2, 0);

        board.update_board();
        assert_eq!(CellState::Dead, board[(1, 1)]);
    }

    #[test]
    fn alive_cell_with_one_alive_neighbour_dies() {
        let mut board = Board::new(3, 3);
        board.toggle_cell_state(1, 1);
        board.toggle_cell_state(1, 0);

        board.update_board();
        assert_eq!(CellState::Dead, board[(1, 1)]);
    }
}
