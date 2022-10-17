use super::{Point, vec3::Vec3};

pub struct Ray {
  pub origin: Point,
  pub direction: Vec3
}

impl Ray {
  fn at(&self, t: f32) -> Point {
    return self.origin + (self.direction * t)
  }
}