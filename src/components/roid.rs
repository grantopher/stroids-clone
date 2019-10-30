
extern crate find_folder;

use opengl_graphics::{Texture, GlGraphics};
use sprite::{Scene, Sprite,};
use piston_window::{TextureSettings, Context, UpdateArgs};
use std::rc::Rc;
use uuid::Uuid;
use crate::utils::{Vector, angle_to_vector, make_sized_bounds};
use crate::{VIEW_H, VIEW_W};
use rand::rngs::ThreadRng;
use rand::Rng;

const MIN_SPEED: f64 = 40.0;
const MAX_SPEED: f64 = 90.0;
const MAX_ROT: f64 = 200.0;
const MIN_SCALE: f64 = 0.5;
const MAX_SCALE: f64 = 1.0;
const MIN_SPAWN_MAG: f64 = 150.0;
const MAX_SPAWN_MAG: f64 = 300.0;


pub struct Roid {
    pos: Vector,
    bounds: Vector,
    vel: Vector,
    size: Vector,
    rot: f64,
    rot_vel: f64,
    dead: bool,
    sprite_id: Uuid,
    diameter: f64,
}

impl Roid {
    pub fn new(tex: Rc<Texture>, scene: &mut Scene<Texture>, rng: &mut ThreadRng) -> Self {
        let mut sprite = Sprite::from_texture_rect(tex, [200.0, 200.0, 50.0, 50.0]);
        let sprite_box = sprite.bounding_box();
        let mag = rng.gen_range(MIN_SPAWN_MAG, MAX_SPAWN_MAG);
        let speed = rng.gen_range(MIN_SPEED, MAX_SPEED);
        let angle = rng.gen_range(0.0, 360.0);
        let scale = rng.gen_range(MIN_SCALE, MAX_SCALE);
        sprite.set_scale(scale, scale);

        Self {
            pos: angle_to_vector(mag, angle),
            rot: 0.0,
            rot_vel: rng.gen_range(-MAX_ROT, MAX_ROT),
            dead: false,
            vel: angle_to_vector(speed, rng.gen_range(0.0, 360.0)),
            sprite_id: scene.add_child(sprite),
            bounds: make_sized_bounds(VIEW_W, VIEW_H, sprite_box),
            size: Vector::new(sprite_box[2], sprite_box[3]),
            diameter: sprite_box[2].max(sprite_box[3]),
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
        self.pos += self.vel * args.dt.into();
        self.rot += self.rot_vel * args.dt;
        if self.pos.x >= VIEW_W + self.diameter {
            self.pos.x = -self.diameter;
        } else if self.pos.x < -self.diameter {
            self.pos.x = VIEW_W + self.diameter;
        }
        if self.pos.y >= VIEW_H + self.diameter {
            self.pos.y = -self.diameter;
        } else if self.pos.y < -self.diameter {
            self.pos.y = VIEW_H + self.diameter;
        }

    }
}