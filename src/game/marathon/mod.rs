use ggez::{ Context, GameResult };
use ggez::event::{ KeyCode, KeyMods };

use super::Game;
use super::super::tetrimino;

fn gen_start_tetriminos() -> [tetrimino::Type; 5] {
    let mut nt = [tetrimino::Type::I; 5];

    for i in 0..5 {
        nt[i] = rand::random::<tetrimino::Type>();
    }

    nt
}

pub struct Marathon {
    pub matrix: [[Option<tetrimino::Type>; 20]; 10],
    pub next_tetriminos: [tetrimino::Type; 5],
}

impl Marathon {
    pub fn new() -> Self {
        Self {
            matrix: [[None; 20]; 10],
            next_tetriminos: gen_start_tetriminos(),
        }
    }
}

impl Game for Marathon {
    fn start(&mut self) -> GameResult {
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn render(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::A | KeyCode::Left => {

            },
            KeyCode::S | KeyCode::Down => {

            },
            KeyCode::D | KeyCode::Right => {

            },
            KeyCode::W | KeyCode::Up | KeyCode::X => {

            },
            KeyCode::Z | KeyCode::LControl => {

            },
            KeyCode::Space => {

            },
            KeyCode::LShift | KeyCode::C => {

            },
            _ => {},
        }
    }
}
