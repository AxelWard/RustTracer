use crate::{math::{vec3::Vec3, ray::Ray}, hittable::HitRecord, color::Color};

pub mod lambertian;

pub trait Material {
  fn scatter(&self, ray_in: &Vec3, hit: &HitRecord) -> Option<(Ray, Color)>;
}