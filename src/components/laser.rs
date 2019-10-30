extern crate find_folder;

use opengl_graphics::{Texture, GlGraphics};
use sprite::{Scene, Sprite,};
use piston_window::{TextureSettings, Context, UpdateArgs};
use std::rc::Rc;
use uuid::Uuid;
use crate::utils::{Vector, angle_to_vector, make_sized_bounds};
use crate::{VIEW_H, VIEW_W};

const LASER_LIFETIME: f64 = 0.8;
const LASER_SPEED: f64 = 8.0;

pub struct Laser {
    pos: Vector,
    bounds: Vector,
    vel: Vector,
    size: Vector,
    rot: f64,
    pub life: f64,
    sprite_id: Uuid,
}

impl Laser {
    pub fn new(pos: Vector, fake_rot: f64, tex: Rc<Texture>, scene: &mut Scene<Texture>) -> Self {
        let mut sprite = Sprite::from_texture_rect(tex, [334.0, 223.0, 4.0, 4.0]);
        let sprite_box = sprite.bounding_box();
        sprite.set_scale(0.5, 0.5);
        let rot = fake_rot - 90.0;

        Self {
            pos,
            rot,
            life: LASER_LIFETIME,
            vel: angle_to_vector(LASER_SPEED, rot),
            sprite_id: scene.add_child(sprite),
            bounds: make_sized_bounds(VIEW_W, VIEW_H, sprite_box),
            size: Vector::new(sprite_box[2], sprite_box[3]),
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
        self.pos += self.vel;
        if self.pos.x >= VIEW_W {
            self.pos.x -= VIEW_W - self.size.x;
        } else if self.pos.x < 0.0 {
            self.pos.y += VIEW_W + self.size.x;
        }
        if self.pos.y >= VIEW_H {
            self.pos.y -= VIEW_H - self.size.y
        } else if self.pos.y < 0.0 {
            self.pos.y += VIEW_H + self.size.y;
        }

        if self.life > 0.0 {
            self.life = (self.life - args.dt).max(0.0);
        }
    }
}