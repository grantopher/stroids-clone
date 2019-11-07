use std::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Sub, SubAssign};

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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
    
    pub fn min(&self, nv: Vector) -> Vector {
        Vector {
            x: self.x.min(nv.x),
            y: self.y.min(nv.y),
        }
    }

    pub fn max(&self, nv: Vector) -> Vector {
        Vector {
            x: self.x.max(nv.x),
            y: self.y.max(nv.y),
        }
    }
    pub fn round(&self) -> Vector {
        Vector {
            x: self.x.round(),
            y: self.y.round(),
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

//pub fn point_in_box(point: Vector, bbox: [Vector; 2]) -> bool {
//    point.x >= bbox[0].x
//    && point.x <= bbox[1].x
//    && point.y >= bbox[0].y
//    && point.y <= bbox[1].y
//}

pub fn point_within_radius(point: Vector, source: Vector, radius: f64) -> bool {
    let delta = ((point.x - source.x).powi(2) + (point.y - source.y).powi(2)).sqrt();
    delta <= radius
}

pub fn loop_pos(pos: Vector, diameter: f64, bounds: Vector) -> Vector {
    let mut ret_vector = Vector::new_empty();
    if pos.x >= bounds.x + diameter {
        ret_vector.x = -diameter;
    } else if pos.x < -diameter {
        ret_vector.x = bounds.x + diameter;
    } else {
        ret_vector.x = pos.x;
    }
    if pos.y >= bounds.y + diameter {
        ret_vector.y = -diameter;
    } else if pos.y < -diameter {
        ret_vector.y = bounds.y + diameter;
    } else {
        ret_vector.y = pos.y;
    }
    ret_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_within_radius() {
        let p1 = Vector::new(100.0, 100.0);
        let p2 = Vector::new(90.0, 90.0);
        assert_eq!(point_within_radius(p1, p2, 15.0), true);
    }
}

