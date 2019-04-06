extern crate ggez;

use ggez::{ event, graphics };
use ggez::{ Context, GameResult };

mod game;
pub use game::Game;

fn main() {
    let cb = ggez::ContextBuilder::new("tetriz", "555555");
    let (ctx, event_loop) = &mut cb.build().unwrap();
    let mut game = Game::new();
    event::run(ctx, event_loop, &mut game).unwrap();
}

// extern crate piston;
// extern crate piston_window;
// extern crate opengl_graphics;
// extern crate graphics;
// extern crate touch_visualizer;

// #[allow(unused_imports)]
// use touch_visualizer::TouchVisualizer;
// use opengl_graphics::{ GlGraphics, OpenGL };
// #[allow(unused_imports)]
// use graphics::{ Context, Graphics };
// #[allow(unused_imports)]
// use std::collections::HashMap;

// use piston_window::{ PistonWindow, WindowSettings };
// use piston::input::*;
// use piston::event_loop::*;

// pub use game::Game;
// pub use game_controller::GameController;
// pub use game_view::{ GameView, GameViewSettings };

// mod game;
// mod game_controller;
// mod game_view;

// fn main() {
//     let opengl = OpenGL::V3_2;
//     let mut window: PistonWindow =
//         WindowSettings::new("tetris", [480, 640])
//         .exit_on_esc(true)
//         .opengl(opengl)
//         .build()
//         .unwrap();

//     let mut gl = GlGraphics::new(opengl);

//     let game = Game::new();
//     let mut game_controller = GameController::new(game);
//     let game_view_settings = GameViewSettings::new();
//     let f = window.factory.clone();

//     let game_view = GameView::new(game_view_settings, f);

//     let mut events = Events::new(EventSettings::new().lazy(true));
//     while let Some(e) = events.next(&mut window) {
//         game_controller.event(&e);

//         if let Some(args) = e.render_args() {
//             gl.draw(args.viewport(), |c, g| {
//                 graphics::clear([1.0; 4], g);
//                 game_view.draw(&game_controller, &c, g);
//             });
//         }
//     }
// }
