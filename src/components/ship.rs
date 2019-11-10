use opengl_graphics::{Texture, GlGraphics};
use sprite::{Scene, Sprite,};
use piston_window::{Context, UpdateArgs};
use serde::Deserialize;
use std::rc::Rc;
use uuid::Uuid;
use crate::utils::{Vector, degree_to_radians, angle_to_vector, loop_pos};
use crate::{VIEW_H, VIEW_W};

#[derive(Deserialize, Clone)]
pub struct ShipConfig {
    scale: f64,
    rotation_increment: f64,
    thrust_increment: f64,
    blink_timer: f64,
    laser_timer: f64,
    max_velocity: f64,
}

pub struct Ship{
    config: ShipConfig,
    sprite_id: Uuid,
    pub pos: Vector,
    pub rot: f64,
    pub vel: Vector,
    pub actions: Actions,
    tinted: bool,
    tint_rgb: [f32; 3],
    blink_cooldown: f64,
    laser_cooldown: f64,
    pub radius: f64,
    pub dead: bool,
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
    pub fn new(config: ShipConfig, scene: &mut Scene<Texture>, tex: Rc<Texture>) -> Self {
        let pos = Vector::new(
            VIEW_W / 2.0,
            VIEW_H / 2.0
        );
        let mut sprite = Sprite::from_texture_rect(tex, [13.0, 4.0, 67.0, 80.0]);
        sprite.set_scale(config.scale, config.scale);
        let sprite_box = sprite.bounding_box();

        Self {
            config,
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
            dead: false,
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut GlGraphics, scene: &mut Scene<Texture>) {
        let sprite = scene.child_mut(self.sprite_id).unwrap();
        let normed = self.pos.round();
        sprite.set_position(normed.x, normed.y);
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
        self.rot += self.config.rotation_increment * delta;
    }
    fn rotate_ccw(&mut self, delta: f64) {
        self.rot -= self.config.rotation_increment * delta;
    }

    fn accelerate(&mut self, delta: f64) {
        let rads = degree_to_radians(self.rot - 90.0);
        let acceleration = Vector::new(
            rads.cos() * self.config.thrust_increment * delta,
            rads.sin() * self.config.thrust_increment * delta,
        );

        self.vel += acceleration;
        self.vel = self.vel.min(self.config.max_velocity.into());
        self.vel = self.vel.max((-1.0 * self.config.max_velocity).into());
    }

    fn blink(&mut self) {
        self.tinted = !self.tinted;
    }

    fn reset_blink_cd(&mut self) {
        self.blink_cooldown = self.config.blink_timer;
    }

    pub fn reset_laser_cd(&mut self) {
        self.laser_cooldown = self.config.laser_timer;
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.pos += self.vel * args.dt.into() * 60.0.into();
        self.pos = loop_pos(self.pos, self.radius, Vector::new(VIEW_W, VIEW_H));

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

    pub fn reset(&mut self) {
        self.vel = Vector::new_empty();
        self.pos = Vector::new(VIEW_W / 2.0, VIEW_H / 2.0);
    }

    pub fn kill(&mut self) {
        self.dead = true;
    }

    pub fn unkill(&mut self) {
        self.dead = false;
    }
}

