use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Circle = 1,
    Cross = 2,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    GameOn = 0,
    CircleWon = 1,
    CrossWon = 2,
    Draw = 3,
}

#[wasm_bindgen]
pub struct Board {
    turn: u32,
    cells: [Cell; 9],
    game_state: GameState,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        Board {
            turn: 0,
            cells: [Cell::Empty; 9],
            game_state: GameState::GameOn,
        }
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn get_index(row: u32, column: u32) -> usize {
        (row * 3 + column) as usize
    }

    pub fn check_box(&mut self, row: u32, column: u32) {
        let index: usize = Self::get_index(row, column);
        if self.cells[index] == Cell::Empty && self.game_state == GameState::GameOn && self.turn < 9
        {
            let cell_to_place: Cell = if self.turn % 2 == 0 {
                Cell::Circle
            } else {
                Cell::Cross
            };

            self.cells[index] = cell_to_place;
            self.turn += 1;
            self.game_state = self.check_game_end();
        }
    }

    fn check_game_end(&mut self) -> GameState {
        let winning_combos: [[usize; 3]; 8] = [
            [0, 1, 2], // first row
            [3, 4, 5], // second row
            [6, 7, 8], // third row
            [0, 3, 6], // first column
            [1, 4, 7], // second column
            [2, 5, 8], // third column
            [0, 4, 8], // first diagonal
            [2, 4, 6], // second diagonal
        ];

        for &combo in &winning_combos {
            let [a, b, c] = combo;
            if self.cells[a] != Cell::Empty
                && self.cells[b] != Cell::Empty
                && self.cells[c] != Cell::Empty
                && self.cells[a] == self.cells[b]
                && self.cells[b] == self.cells[c]
            {
                if self.cells[a] == Cell::Circle {
                    return GameState::CircleWon;
                } else {
                    return GameState::CrossWon;
                }
            }
        }
        if self.turn == 9 {
            return GameState::Draw;
        }
        return GameState::GameOn;
    }
}
