
extern crate find_folder;

use opengl_graphics::{Texture, GlGraphics};
use sprite::{Scene, Sprite,};
use piston_window::{Context, UpdateArgs};
use std::rc::Rc;
use uuid::Uuid;
use crate::utils::{Vector, angle_to_vector, loop_pos, point_within_radius};
use crate::{VIEW_H, VIEW_W};
use rand::rngs::ThreadRng;
use serde::Deserialize;
use rand::Rng;

#[derive(Clone, Deserialize)]
pub struct RoidConfig {
    min_speed: f64,
    max_speed: f64,
    max_rot: f64,
    min_scale: f64,
    max_scale: f64,
    min_spawn_mag: f64,
    max_spawn_mag: f64,
}


pub struct Roid {
    pub pos: Vector,
    vel: Vector,
    rot: f64,
    rot_vel: f64,
    sprite_id: Uuid,
    pub diameter: f64,
}

impl Roid {
    pub fn new(config: RoidConfig, tex: Rc<Texture>, scene: &mut Scene<Texture>, rng: &mut ThreadRng) -> Self {
        let mut sprite = Sprite::from_texture_rect(tex, [200.0, 200.0, 50.0, 50.0]);
        let mut pos = rng_vector(rng);
        while point_within_radius(pos, Vector::new(VIEW_W / 2.0, VIEW_H / 2.0), config.min_spawn_mag) {
            pos = rng_vector(rng);
        }
        let sprite_box = sprite.bounding_box();
        let speed = rng.gen_range(config.min_speed, config.max_speed);
        let scale = rng.gen_range(config.min_scale, config.max_scale);
        sprite.set_scale(scale, scale);

        Self {
            pos,
            rot: 0.0,
            rot_vel: rng.gen_range(-config.max_rot, config.max_rot),
            vel: angle_to_vector(speed, rng.gen_range(0.0, 360.0)),
            sprite_id: scene.add_child(sprite),
            diameter: sprite_box[2].max(sprite_box[3]) * scale,
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut GlGraphics, scene: &mut Scene<Texture>) {
        let sprite = scene.child_mut(self.sprite_id).unwrap();
        sprite.set_position(self.pos.x, self.pos.y);
        sprite.set_rotation(self.rot);
        sprite.draw(
            context.transform,
            graphics
        );
    }
    pub fn update(&mut self, args: UpdateArgs) {
        self.pos += self.vel * args.dt.into() * 60.0.into();
        self.rot += self.rot_vel * args.dt;
        self.pos = loop_pos(self.pos, self.diameter, Vector::new(VIEW_W, VIEW_H));
    }
}

fn rng_vector(rng: &mut ThreadRng) -> Vector {
    Vector {
        x: rng.gen_range(0.0, VIEW_W),
        y: rng.gen_range(0.0, VIEW_H),
    }
}