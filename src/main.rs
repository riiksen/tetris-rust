extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate touch_visualizer;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

use touch_visualizer::TouchVisualizer;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{ Context, Graphics };
use std::collections::HashMap;

use piston::window::{ AdvancedWindow, Window, WindowSettings };
use piston::input::*;
use piston::event_loop::*;

#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

pub use game::Game;
pub use game_controller::GameController;
pub use game_view::{ GameView, GameViewSettings };

mod game;
mod game_controller;
mod game_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow =
        WindowSettings::new("tetris", [480, 640])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let game = Game::new();
    let mut game_controller = GameController::new(game);
    let game_view_settings = GameViewSettings::new();
    let game_view = GameView::new(game_view_settings);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        game_controller.event(&e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                game_view.draw(&game_controller, &c, g);
            });
        }
    }
}
