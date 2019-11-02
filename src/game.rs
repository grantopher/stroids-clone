use piston_window::{PistonWindow, clear, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent, Button, Key, TextureSettings};
use opengl_graphics::{GlGraphics, Texture};
use sprite::{Scene};
use serde::Deserialize;
use std::rc::Rc;


use crate::utils::{BLACK, point_within_radius};
use crate::GameConfig;
use crate::components::ship::Ship;
use crate::components::laser::Laser;
use crate::components::roid::{Roid, RoidConfig};
use rand::rngs::ThreadRng;
use rand::{thread_rng};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct KeyConfig {
    rotate_cw: Key,
    rotate_ccw: Key,
    fire_laser: Key,
    thrust: Key,
}

#[derive(Deserialize)]
pub struct GeneratorConfig {
    num_of_asteroids: i32,
}

pub struct Game {
    lasers: Vec<Laser>,
    roids: Vec<Roid>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            lasers: Vec::new(),
            roids: Vec::new(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, scene: &mut Scene<Texture>, config: GameConfig, config_dir: PathBuf) {
        let mut rng = thread_rng();
        let texture = load_texture(config_dir);
        let mut ship = Ship::new(config.ship_config.clone(), scene, texture.clone());
        self.roids = generate_roids(scene, texture.clone(), &mut rng, config.generator_config.num_of_asteroids, config.roid_config.clone());
        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                opengl.draw(args.viewport(), |context, graphics| {
                    clear(BLACK,graphics);
                    ship.draw(context, graphics, scene);
                    for laser in &mut self.lasers {
                        laser.draw(context, graphics, scene);
                    }
                    for roid in &mut self.roids {
                        roid.draw(context, graphics, scene);
                    }
                });
            }
            if let Some(args) = event.update_args() {
                ship.update(args);
                if ship.is_firing_laser() {
                    self.lasers.push(Laser::new(
                        config.laser_config.clone(),
                        ship.get_laser_pos(),
                        ship.rot.clone(),
                        texture.clone(),
                        scene,
                    ));
                    ship.reset_laser_cd();
                }

                for laser in &mut self.lasers {
                    laser.update(args);
                }
                self.lasers.retain(|laser| laser.life > 0.0);
                for roid in &mut self.roids {
                    roid.update(args);
                }
                let mut l_copy = self.lasers.clone();
                self.roids.retain(|roid| {
                    let mut colide = false;
                    for l in &mut l_copy {
                        if point_within_radius(l.pos, roid.pos, roid.diameter / 2.0) {
                            colide = true;
                            break;
                        }
                    }
                    !colide
                });
            }
            if let Some(Button::Keyboard(key)) = event.press_args() {
                if key == config.key_config.rotate_cw {
                    ship.actions.rotate_cw  = true;
                }
                if key == config.key_config.rotate_ccw {
                    ship.actions.rotate_ccw = true;
                }
                if key == config.key_config.thrust {
                    ship.actions.fire_boosters = true;
                }
                if key == config.key_config.fire_laser {
                    ship.actions.is_shooting = true;
                }
            }

            if let Some(Button::Keyboard(key)) = event.release_args() {
                if key == config.key_config.rotate_cw {
                    ship.actions.rotate_cw  = false;
                }
                if key == config.key_config.rotate_ccw {
                    ship.actions.rotate_ccw = false;
                }
                if key == config.key_config.thrust {
                    ship.actions.fire_boosters = false;
                }
                if key == config.key_config.fire_laser {
                    ship.actions.is_shooting = false;
                }
            }
        }
    }
}

fn load_texture(config_dir: PathBuf) -> Rc<Texture> {
    let mut ship_asset_path = config_dir;
    ship_asset_path.push("img");
    ship_asset_path.push("ship2.png");
    Rc::new(
        Texture::from_path(
            ship_asset_path,
            &TextureSettings::new(),
        ).unwrap(),
    )
}

fn generate_roids(scene: &mut Scene<Texture>, texture: Rc<Texture>, rng: &mut ThreadRng, n: i32, config: RoidConfig) -> Vec<Roid> {
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(Roid::new(config.clone(), texture.clone(), scene, rng));
    }
    v
}