#[derive(Clone)]
enum CellState {
    Alive,
    Dead
}
struct Board(Vec<Vec<(CellState, CellState)>>);

impl Board {
    fn new() -> Self {
        let mut row = vec![(CellState::Dead, CellState::Dead); BOARD_WIDTH];
        let mut board = vec![row; BOARD_LENGTH];
        board
    }
}

const BOARD_LENGTH: usize = 10;
const BOARD_WIDTH: usize = 10;


fn main() {
    let mut game_board = Board::new();
    
}
