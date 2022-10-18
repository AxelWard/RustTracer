use crate::{color::Color, objects::Sphere};

use super::{ray::Ray, vec3::Vec3};

pub fn lerp(t: f32, min: &Color, max: &Color) -> Color {
  return *min * (1.0 - t) + *max * t;
}

pub fn hit_sphere(s: &Sphere, r: &Ray) -> f32 {
  let oc: Vec3 = r.origin - s.center;
  let a: f32 = r.direction.length_squared();
  let half_b: f32 = oc.dot(&r.direction);
  let c: f32 = oc.length_squared() - s.radius * s.radius;
  let descriminant: f32 = half_b * half_b - a * c;

  if descriminant < 0.0 {
    return -1.0;
  } else {
    return (-half_b - descriminant.sqrt()) / a;
  }
}