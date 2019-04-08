use ggez::{ Context, GameResult };
use ggez::event::{ KeyCode, KeyMods };

use super::Game;
use super::super::tetrimino;

mod render;

fn gen_start_tetriminos() -> [tetrimino::Type; 5] {
    let mut nt = [tetrimino::Type::I; 5];

    for i in 0..5 {
        nt[i] = rand::random::<tetrimino::Type>();
    }

    nt
}

pub struct Marathon {
    pub view_settings: render::ViewSettings,

    pub matrix: [[Option<tetrimino::Type>; 20]; 10],
    pub next_tetriminos: [tetrimino::Type; 5],
    pub holding: Option<tetrimino::Type>,
    pub current_tetrimino: tetrimino::Type,
    pub current_column: u8,

    pub finished: bool,
}

impl Marathon {
    pub fn new() -> Self {
        Self {
            view_settings: render::ViewSettings::new(),

            matrix: [[None; 20]; 10],
            next_tetriminos: gen_start_tetriminos(),
            holding: None,
            current_tetrimino: tetrimino::random(),
            current_column: 5,
            finished: false,
        }
    }

    fn end(&mut self) {
        self.finished = true;
    }

    // TODO: Implement
    fn move_left(&mut self) {

    }

    // TODO: Implement
    fn move_right(&mut self) {

    }

    // TODO: Implement
    fn move_down(&mut self) {

    }

    // TODO: Implement
    fn hard_drop(&mut self) {

    }

    // TODO: Implement
    fn hold(&mut self) {

    }

    // TODO: Implement
    fn rotate_left(&mut self) {

    }

    // TODO: Implement
    fn rotate_right(&mut self) {

    }
}

impl Game for Marathon {
    fn start(&mut self) -> GameResult {
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn is_finished(&mut self) -> bool {
        self.finished
    }

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        render::render_board(ctx, &self.view_settings);
        render::render_current_next_and_holding(ctx, &self.view_settings);
        render::render_state(ctx, &self.view_settings, &self);
        render::render_current_and_shadow(ctx, &self.view_settings);

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
                self.move_left();
            },
            KeyCode::S | KeyCode::Down => {
                self.move_down();
            },
            KeyCode::D | KeyCode::Right => {
                self.move_right();
            },
            KeyCode::W | KeyCode::Up | KeyCode::X => {
                self.rotate_right();
            },
            KeyCode::Z | KeyCode::LControl => {
                self.rotate_left();
            },
            KeyCode::Space => {
                self.hard_drop();
            },
            KeyCode::LShift | KeyCode::C => {
                self.hold();
            },
            KeyCode::Escape => {
                self.end();
            },
            _ => {},
        }
    }
}
