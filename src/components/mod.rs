use piston_window::{Context};
use opengl_graphics::GlGraphics;

pub mod ship;
pub mod laser;
pub mod roid;

pub trait Drawable {
    fn draw(&self, context: Context, graphics: &mut  GlGraphics);
}