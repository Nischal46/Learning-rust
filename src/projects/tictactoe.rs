#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    X,
    O
}

enum Player {
    X,
    O
}

impl Player {
    fn to_cell(&self) -> Cell {
        match self {
            Player::X => Cell::X,
            Player::O => Cell::O
        }
    }

    fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X
        }
    }
}

enum GameState {
    InProgress,
    Draw,
    Won(Player)
}


struct Board {
     cells: [Cell; 9],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [Cell::Empty; 9]
        }
    } 

    fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }

    fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row * 3 + col]
    }
}