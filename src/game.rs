use piston_window::{PistonWindow, clear, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent, Button, Key, TextureSettings, text, Transformed};
use opengl_graphics::{GlGraphics, Texture, GlyphCache};
use sprite::{Scene};
use serde::Deserialize;
use std::{rc::Rc, time::SystemTime};


use crate::utils::{BLACK, point_within_radius};
use crate::GameConfig;
use crate::components::ship::Ship;
use crate::components::laser::Laser;
use crate::components::roid::{Roid, RoidConfig};
use rand::rngs::ThreadRng;
use rand::{thread_rng};

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

struct TickCounter {
    input: u64,
    render: u64,
    update: u64,
    last_tick: SystemTime,
    first_frame: SystemTime,
    render_frames: f64,
    avg_render_frames: f64,
    update_ticks: f64,
}

impl TickCounter {
    fn new(input: u64, render: u64, update: u64) -> Self{
        let now = SystemTime::now();
        Self {
            input, render, update,
            last_tick: now,
            first_frame: now,
            render_frames: 0.0,
            avg_render_frames: 0.0,
            update_ticks: 0.0,
            avg_update_ticks: 0.0,
        }
    }
    fn tick_input(&mut self) {
        self.input += 1;
    }
    fn tick_render(&mut self) {
        self.render += 1;
    }
    fn tick_update(&mut self) {
        self.update += 1;
    }
    fn run(&mut self) {
        let delta_last = self.last_tick.elapsed().unwrap();
        let delta_total = self.first_frame.elapsed().unwrap();
        if delta_last.as_secs() == 0 {
            return
        }
        self.render_frames = delta_last.as_secs_f64();
        self.avg_render_frames = self.render as f64 / delta_total.as_secs_f64();
        self.last_tick = SystemTime::now();
        self.update_ticks = self.update as f64 / delta_total.as_secs_f64();
    }

    fn formatted(&self) -> String {
        format!("FPS: {} TPS: {} ELP: {}", self.avg_render_frames, self.update_ticks, self.first_frame.elapsed().unwrap().as_secs_f64().floor())
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            lasers: Vec::new(),
            roids: Vec::new(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, scene: &mut Scene<Texture>, config: GameConfig) {
        let mut rng = thread_rng();
        let texture = load_texture();
        let mut level = 1;
        let mut glyph_cache = GlyphCache::new("./assets/fonts/square.ttf", (), TextureSettings::new()).unwrap();
        let mut ship = Ship::new(config.ship_config.clone(), scene, texture.clone());
        let mut roids = generate_roids(scene, texture.clone(), &mut rng, config.generator_config.num_of_asteroids, config.roid_config.clone());
        let mut lasers: Vec<Laser> = Vec::new();
        let mut tick_counter = TickCounter::new(0, 0, 0);
        let mut score = 0;
        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                opengl.draw(args.viewport(), |context, graphics| {
                    clear(BLACK,graphics);
                    ship.draw(context, graphics, scene);
                    for laser in &mut lasers {
                        laser.draw(context, graphics, scene);
                    }
                    for roid in &mut roids {
                        roid.draw(context, graphics, scene);
                    }
                    text([1.0; 4], 14, &format!("Targets Remaining: {} Score: {}", roids.len(), score), &mut glyph_cache, context.transform.trans(50.0, 50.0), graphics);
                });
                tick_counter.tick_render();
            }
            if let Some(args) = event.update_args() {
                ship.update(args);
                if ship.is_firing_laser() {
                    lasers.push(Laser::new(
                        ship.vel.clone(),
                        config.laser_config.clone(),
                        ship.get_laser_pos(),
                        ship.rot.clone(),
                        texture.clone(),
                        scene,
                    ));
                    ship.reset_laser_cd();
                }

                for laser in &mut lasers {
                    laser.update(args);
                }
                // loop a
                for roid in &mut roids {
                    roid.update(args);
                }
                //loop
                roids.retain(|roid| {
                    let mut keep_asteroid = true;
                    //loop a1
                    lasers.retain(|l| {
                        if point_within_radius(l.pos, roid.pos, roid.diameter / 2.0) {
                            keep_asteroid = false;
                            score += roid.diameter.round() as u32;
                        }
                        l.life > 0.0 && keep_asteroid
                    });
                    if point_within_radius(ship.pos, roid.pos, roid.diameter / 2.0 + ship.radius) {
                        ship.kill();
                        println!("killing the ship");
                    }
                    keep_asteroid
                });
                tick_counter.tick_update();
                if roids.len() == 0 && !ship.dead {
                    ship.reset();
                    level += 1;
                    let num_of_roids = config.generator_config.num_of_asteroids + level * level;
                    roids = generate_roids(scene, texture.clone(), &mut rng, num_of_roids, config.roid_config.clone());
                } else if ship.dead {
                    ship.reset();
                    level = 1;
                    roids = generate_roids(scene, texture.clone(), &mut rng, config.generator_config.num_of_asteroids, config.roid_config.clone());
                    score = 0;
                    ship.unkill();
                }
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
                tick_counter.tick_input();
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
            tick_counter.run();
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

fn generate_roids(scene: &mut Scene<Texture>, texture: Rc<Texture>, rng: &mut ThreadRng, n: i32, config: RoidConfig) -> Vec<Roid> {
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(Roid::new(config.clone(), texture.clone(), scene, rng));
    }
    v
}