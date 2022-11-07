use crate::{
  color::Color, 
  math::{util::random_on_hemisphere, ray::Ray}, hittable::HitRecord
};

use super::Material;

pub struct Lambertian {
  pub albedo: Color
}

impl Material for Lambertian {
  fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
    let mut direction = hit.p + random_on_hemisphere(&hit.normal);
    if direction.near_zero() {
      direction = hit.normal;
    }

    return Some((Ray { origin: hit.p, direction: direction.unit() - hit.p }, self.albedo));
  }
}