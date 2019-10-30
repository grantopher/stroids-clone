use std::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Sub, SubAssign};

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0; 4];

#[derive(Copy, Clone, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new_empty() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl Rem for Vector {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Vector {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl RemAssign for Vector {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl From<f64> for Vector {
    fn from(num: f64) -> Self {
        Vector {
            x: num,
            y: num,
        }
    }
}

pub fn degree_to_radians(degrees: f64) -> f64 {
    (degrees / 180.0) * std::f64::consts::PI
}

pub fn angle_to_vector(mag: f64, theta: f64) -> Vector {
    let r_rad = degree_to_radians(theta);
    Vector::new(
        r_rad.cos() * mag,
        r_rad.sin() * mag,
    )
}

pub fn make_sized_bounds(width: f64, height: f64, sprite_box: [f64; 4]) -> Vector {
    Vector {
        x: width + sprite_box[2] * 2.0,
        y: height + sprite_box[3] * 2.0,
    }
}