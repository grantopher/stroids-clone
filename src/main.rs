extern crate find_folder;

use opengl_graphics::{GlGraphics, Texture};
use sprite::*;
use piston_window::{OpenGL, Size, WindowSettings};
use std::{fs::File};
use serde::Deserialize;
use ron::de::from_reader;

mod game;
mod utils;
mod components;

use game::Game;
use crate::components::{ship::ShipConfig, laser::LaserConfig, roid::RoidConfig};
use crate::game::{KeyConfig, GeneratorConfig};

const VIEW_W: f64 = 1024.0;
const VIEW_H: f64 = 768.0;

fn main() {
    let game_config = load_cfg();
    let game_title = "Stroids...";
    let game_window_size = Size {
        width: VIEW_W,
        height: VIEW_H,
    };
    let opengl = OpenGL::V3_2;

    let mut window = WindowSettings::new(
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
    game.run(&mut window, &mut gl, &mut scene, game_config);

}

#[derive(Deserialize)]
pub struct GameConfig {
    ship_config: ShipConfig,
    key_config: KeyConfig,
    generator_config: GeneratorConfig,
    laser_config: LaserConfig,
    roid_config: RoidConfig,
}

fn load_cfg() -> GameConfig {
    let input_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path).expect("Failed to open file");
    match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
}
