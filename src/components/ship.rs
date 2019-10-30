extern crate find_folder;

use opengl_graphics::{Texture, GlGraphics};
use sprite::{Scene, Sprite,};
use piston_window::{Context, UpdateArgs};
use std::rc::Rc;
use uuid::Uuid;
use crate::utils::{Vector, degree_to_radians, angle_to_vector};
use crate::{VIEW_H, VIEW_W};

const SHIP_SCALE: f64 = 0.5;
const ROTATION_INCREMENT: f64 = 1000.0;
const THRUST_INCREMENT: f64 = 8.0;
const BLINK_TIMER: f64 = 0.04;
const LASER_TIMER: f64 = 0.05;

pub struct Ship {
    sprite_id: Uuid,
    pub pos: Vector,
    pub rot: f64,
    vel: Vector,
    pub actions: Actions,
    tinted: bool,
    tint_rgb: [f32; 3],
    blink_cooldown: f64,
    laser_cooldown: f64,
    radius: f64,
    bounds: Vector,

}

#[derive(Default)]
pub struct Actions {
    pub rotate_cw: bool,
    pub rotate_ccw: bool,
    pub fire_boosters: bool,
    pub is_shooting: bool,
    pub is_blinking: bool,
}

impl Ship {
    pub fn new(scene: &mut Scene<Texture>, tex: Rc<Texture>) -> Self {
        let pos = Vector::new(
            VIEW_W / 2.0,
            VIEW_H / 2.0
        );
        let mut sprite = Sprite::from_texture_rect(tex, [13.0, 4.0, 67.0, 80.0]);
        sprite.set_scale(SHIP_SCALE, SHIP_SCALE);
        let sprite_box = sprite.bounding_box();

        Self {
            sprite_id: scene.add_child(sprite),
            pos,
            rot: 0.0,
            vel: Vector::new_empty(),
            actions: Actions::default(),
            tinted: false,
            tint_rgb: [1.0, 0.0, 0.0],
            blink_cooldown: 0.0,
            laser_cooldown: 0.0,
            radius: sprite_box[2].max(sprite_box[3]),
            bounds: Vector::new(
                VIEW_W + sprite_box[2] * 2.0,
                VIEW_H + sprite_box[3] * 2.0,
            )

        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut GlGraphics, scene: &mut Scene<Texture>) {
        let sprite = scene.child_mut(self.sprite_id).unwrap();
        sprite.set_position(self.pos.x, self.pos.y);
        sprite.set_rotation(self.rot);
        if self.tinted {
            sprite.draw_tinted(
                context.transform,
                graphics,
                self.tint_rgb,
            );
        } else {
            sprite.draw(
                context.transform,
                graphics
            );
        }

    }

    fn rotate_cw(&mut self, delta: f64) {
        self.rot += ROTATION_INCREMENT * delta;
    }
    fn rotate_ccw(&mut self, delta: f64) {
        self.rot -= ROTATION_INCREMENT * delta;
    }

    fn accelerate(&mut self, delta: f64) {
        let rads = degree_to_radians(self.rot - 90.0);
        let acceleration = Vector::new(
            rads.cos() * THRUST_INCREMENT * delta,
            rads.sin() * THRUST_INCREMENT * delta,
        );

        self.vel += acceleration;
    }

    fn blink(&mut self) {
        self.tinted = !self.tinted;
    }

    fn reset_blink_cd(&mut self) {
        self.blink_cooldown = BLINK_TIMER;
    }

    pub fn reset_laser_cd(&mut self) {
        self.laser_cooldown = LASER_TIMER;
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.pos += self.vel;
        if self.pos.x >= VIEW_W + self.radius {
            self.pos.x = -self.radius;
        } else if self.pos.x < -self.radius {
            self.pos.x = VIEW_W + self.radius;
        }
        if self.pos.y >= VIEW_H + self.radius {
            self.pos.y = -self.radius;
        } else if self.pos.y < -self.radius {
            self.pos.y = VIEW_H + self.radius;
        }

        if self.actions.rotate_cw {
            self.rotate_cw(args.dt);
        }
        if self.actions.rotate_ccw {
            self.rotate_ccw(args.dt);
        }
        if self.actions.fire_boosters {
            self.accelerate(args.dt);
        }
        if self.actions.is_blinking && self.blink_cooldown == 0.0 {
            self.blink();
            self.reset_blink_cd();
        }

        if self.blink_cooldown > 0.0 {
            self.blink_cooldown = (self.blink_cooldown - args.dt).max(0.0);
        }

        if self.laser_cooldown > 0.0 {
            self.laser_cooldown = (self.laser_cooldown - args.dt).max(0.0);
        }
    } 

    pub fn is_firing_laser(&mut self) -> bool {
        self.laser_cooldown == 0.0 && self.actions.is_shooting
    }
    
    pub fn get_laser_pos(&mut self) -> Vector {
        angle_to_vector(self.radius / 2.0, self.rot - 90.0) + self.pos
    }
}

