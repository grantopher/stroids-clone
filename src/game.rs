use piston_window::{PistonWindow, clear, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent, Button, Key, TextureSettings};
use opengl_graphics::{GlGraphics, Texture};
use sprite::{Scene};
use std::rc::Rc;

use crate::utils::BLACK;
use crate::components::ship::Ship;
use crate::components::laser::Laser;
use crate::components::roid::Roid;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::{thread_rng};

pub struct Game {
    score: i128,
    timer: f64,
    lasers: Vec<Laser>,
    roids: Vec<Roid>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            score: 0,
            timer: 0.0,
            lasers: Vec::new(),
            roids: Vec::new(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, scene: &mut Scene<Texture>) {
        let mut rng = thread_rng();
        let texture = load_texture();
        let mut ship = Ship::new(scene, texture.clone());
        self.roids = generate_roids(scene, texture.clone(), &mut rng);
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
            }
            if let Some(Button::Keyboard(key)) = event.press_args() {
                match key {
                    Key::A => ship.actions.rotate_ccw = true,
                    Key::S => ship.actions.rotate_cw = true,
                    Key::W => ship.actions.fire_boosters = true,
                    Key::Space => ship.actions.is_shooting = true,
                    _ => {}
                }
            }

            if let Some(Button::Keyboard(key)) = event.release_args() {
                match key {
                    Key::A => ship.actions.rotate_ccw = false,
                    Key::S => ship.actions.rotate_cw = false,
                    Key::W => ship.actions.fire_boosters = false,
                    Key::Space => ship.actions.is_shooting = false,
                    _ => {}
                }
            }
        }
    }
}

fn load_texture() -> Rc<Texture> {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    Rc::new(
        Texture::from_path(
            assets.join("img/ship2.png"),
            &TextureSettings::new(),
        ).unwrap(),
    )
}

fn generate_roids(scene: &mut Scene<Texture>, texture: Rc<Texture>, rng: &mut ThreadRng) -> Vec<Roid> {
    let mut v = Vec::new();
    for i in 0..4000 {
        v.push(Roid::new(texture.clone(), scene, rng));
    }
    v
}