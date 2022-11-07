use std::rc::Rc;

use crate::{math::ray::Ray, hittable::HitRecord, color::Color};

pub mod lambertian;

pub trait Material {
  fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

impl <S: Material + ?Sized> Material for Rc<S> {
  fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
    return (**self).scatter(ray_in, hit);
  }
}