use opengl_graphics::{GlGraphics, Texture};
use sprite::*;
use piston_window::{OpenGL, PistonWindow, Size, WindowSettings};

mod game;
mod utils;
mod components;

use game::Game;

const VIEW_W: f64 = 1024.0;
const VIEW_H: f64 = 768.0;

fn main() {
    let game_title = "Stroids...";
    let game_window_size = Size {
        width: VIEW_W,
        height: VIEW_H,
    };
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        game_title,
        [game_window_size.width, game_window_size.height],
    )
    .samples(4)
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    let mut scene = Scene::<Texture>::new();

    let mut game = Game::new();
    let mut gl = GlGraphics::new(opengl);
    game.run(&mut window, &mut gl, &mut scene);

}
