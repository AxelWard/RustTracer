use crate::{
  color::Color, 
  math::{vec3::Vec3, Point, util::random_on_hemisphere, ray::Ray}, hittable::HitRecord
};

use super::Material;

pub struct Lambertian {
  pub albedo: Color
}

impl Material for Lambertian {
  fn scatter(&self, ray_in: &Vec3, hit: &HitRecord) -> Option<(Ray, Color)> {
    let target: Point = hit.p + random_on_hemisphere(&hit.normal);
    return Some((Ray { origin: hit.p, direction: target - hit.p }, self.albedo));
  }
}