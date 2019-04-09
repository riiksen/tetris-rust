use ggez::{ Context, GameResult };
use ggez::event::{ KeyCode, KeyMods };

use super::Game;
use super::super::tetrimino;

mod view;

use view::MarathonView;

fn gen_start_tetriminos() -> Vec<tetrimino::Type> {
    let mut nt = Vec::new();

    for _ in 0..5 {
        nt.push(rand::random::<tetrimino::Type>());
    }

    nt
}

pub struct Marathon {
    view: MarathonView,

    pub matrix: [[Option<tetrimino::Type>; 20]; 10],
    pub next_tetriminos: Vec<tetrimino::Type>,
    pub holding: Option<tetrimino::Type>,
    pub current_tetrimino: tetrimino::Type,
    pub current_column: u8,

    pub finished: bool,
}

impl Marathon {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            view: MarathonView::new(ctx),

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

    fn move_left(&mut self) {
        if self.current_column > 0 {
            self.current_column -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.current_column < 10 {
            self.current_column += 1;
        }
    }

    // TODO: Implement
    fn move_down(&mut self) {

    }

    // TODO: Implement
    fn hard_drop(&mut self) {

    }

    fn hold(&mut self) {
        if let None = self.holding {
            self.holding = Some(self.current_tetrimino);

            self.current_tetrimino = self.next_tetriminos[0];
            self.next_tetriminos.remove(0);
            self.next_tetriminos.push(rand::random::<tetrimino::Type>());
        } else {
            let switch_tmp = self.current_tetrimino;
            self.current_tetrimino = self.holding.unwrap();
            self.holding = Some(switch_tmp);

        }
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
        self.view.render_board(ctx);
        self.view.render_current_next_and_holding(ctx);

        self.view.render_state(ctx, (self.matrix, self.current_tetrimino, &self.next_tetriminos, self.holding));
        self.view.render_current_and_shadow(ctx, (self.current_tetrimino, self.current_column));

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
