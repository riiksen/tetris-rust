//! Board logic

pub struct Board {
    pub cells: [[u8; 20]; 10],
    pub next_tetriminos: [u8; 5],
    pub holding: u8,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [[0; 20]; 10],
            next_tetriminos: [0; 5],
            holding: 0,
        }
    }

    pub fn gen_new_tetrimino(&self) {

    }
}

pub struct Tetrimino {
}


