extern crate find_folder;

use opengl_graphics::{Texture, GlGraphics};
use sprite::{Scene, Sprite,};
use piston_window::{Context, UpdateArgs};
use std::rc::Rc;
use serde::Deserialize;
use uuid::Uuid;
use crate::utils::{Vector, angle_to_vector, loop_pos};
use crate::{VIEW_H, VIEW_W};

#[derive(Deserialize, Clone)]
pub struct LaserConfig {
    laser_lifetime: f64,
    laser_speed: f64,
}

#[derive(Clone)]
pub struct Laser {
    pub pos: Vector,
    vel: Vector,
    size: Vector,
    rot: f64,
    pub life: f64,
    sprite_id: Uuid,
    diameter: f64,
}

impl Laser {
    pub fn new(config: LaserConfig, pos: Vector, fake_rot: f64, tex: Rc<Texture>, scene: &mut Scene<Texture>) -> Self {
        let mut sprite = Sprite::from_texture_rect(tex, [334.0, 223.0, 4.0, 4.0]);
        let sprite_box = sprite.bounding_box();
        sprite.set_scale(0.5, 0.5);
        let rot = fake_rot - 90.0;

        Self {
            pos,
            rot,
            life: config.laser_lifetime,
            vel: angle_to_vector(config.laser_speed, rot),
            sprite_id: scene.add_child(sprite),
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
        self.pos = loop_pos(self.pos, self.diameter, Vector::new(VIEW_W, VIEW_H));

        if self.life > 0.0 {
            self.life = (self.life - args.dt).max(0.0);
        }
    }
}