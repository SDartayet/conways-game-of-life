#[derive(Clone, PartialEq)]
enum CellState {
    Alive,
    Dead
}

#[derive(Clone)]
struct Board(Vec<Vec<(CellState)>>);

impl Board {
    fn new() -> Self {
        let mut row = vec![(CellState::Dead); BOARD_WIDTH];
        let mut board = vec![row; BOARD_LENGTH];
        Board(board)
    }

    fn swap_cell_state(&mut self, row: usize, col: usize) {
        if self.0[col][row] == CellState::Alive { self.0[col][row] = CellState::Dead; } else { self.0[col][row] = CellState::Alive ;}
    }

    fn update_board (&mut self) {
        let board_old = &self.clone();
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                self.update_cell_state(board_old, x,y);
            }
        }
    }

    fn update_cell_state (&self, old_board:&Board, x: usize, y: usize) {
        let x_offsets: std::ops::Range<i8> = if x > 1 { -1..1 } else { 0..1 };
        let y_offsets: std::ops::Range<i8> = if y > 1 { -1..1 } else { 0..1 };
        
        let mut alive_neighbours:u8 = 0;
        for x_offset in x_offsets {
            for y_offset in y_offsets.clone() {
                if (x_offset == 0 && y_offset == 0) { continue; }
                if (old_board.0[y + y_offset][x + x_offset].1 == CellState::Alive ) { alive_neighbours += 1; }
                match alive_neighbours {
                    0..2 => { self.0[y][x] = CellState::Dead; },
                    2..3 => {},
                    3..5 => { self.0[y][x] = CellState::Alive; },
                    5.. => { self.0[y][x] = CellState::Dead; }
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


