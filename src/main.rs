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
use opengl_graphics::{ GLGraphics, OpenGL };
use graphics::{ Context, Graphics };
use std::collections::HashMap;
use piston::window::*;

#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

fn main() {
    let mut window: AppWindow =
        WindowSettings::new("tetris", [480, 640])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {

    }




    println!("Hello, world!");
}
