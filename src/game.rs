//! Game logic
extern crate rand;

use opengl_graphics::{ GlGraphics, OpenGL };

use rand::prelude::*;

pub struct App {
    board: [[u8; 20]; 10],
    next_tetriminos: [Tetrimino; 5],
    holding: u8,
}

impl App {
    pub fn new() -> App {
        App {
            board: [[0; 20]; 10],
            next_tetriminos: [Tetrimino::I; 5],
            holding: 0,
        }
    }

    pub fn update(&mut self) {

    }

    pub fn gen_new_tetrimino(&mut self) {
        let _x: u8 = random();
    }
}

#[derive(Copy, Clone)]
pub enum Tetrimino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}
