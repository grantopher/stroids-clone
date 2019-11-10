#[macro_use]
extern crate structopt;

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
use std::path::PathBuf;
use structopt::StructOpt;

const VIEW_W: f64 = 1024.0;
const VIEW_H: f64 = 768.0;

#[derive(Debug, StructOpt)]
#[structopt(name = "stroids", about = "stroids.")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str), env = "STROIDS_CONFIG_PATH")]
    pub config: Option<PathBuf>,
    #[structopt(short, long, parse(from_os_str), env = "STROIDS_ASSETS_PATH")]
    pub assets: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let config_dir = Option::<PathBuf>::expect(opt.config, "No path to config dir");
    let assets_dir = Option::<PathBuf>::expect(opt.assets, "No path to assets dir");
    let game_config = load_cfg(config_dir.clone());
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
    game.run(&mut window, &mut gl, &mut scene, game_config, assets_dir.clone());
}

#[derive(Deserialize)]
pub struct GameConfig {
    ship_config: ShipConfig,
    key_config: KeyConfig,
    generator_config: GeneratorConfig,
    laser_config: LaserConfig,
    roid_config: RoidConfig,
}

fn load_cfg(mut config_dir: PathBuf) -> GameConfig {
    config_dir.push("config.ron");
    let f = File::open(config_dir).expect("Failed to open file");
    match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
}
