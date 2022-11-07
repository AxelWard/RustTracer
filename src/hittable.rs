use std::{rc::Rc};

use crate::{math::{Point, vec3::Vec3, ray::Ray}, material::Material};

pub mod sphere;

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
  pub p: Point,
  pub normal: Vec3,
  pub t: f32,
  pub front_face: bool,
  pub material: Rc<dyn Material>
}

impl HitRecord {
  pub fn set_front_face(&mut self, ray: &Ray, outward_normal: &Vec3) {
    self.front_face = ray.direction.dot(outward_normal) < 0.0;
    if self.front_face {
      self.normal = *outward_normal;
    }  else {
      self.normal = -*outward_normal;
    } 
  }
}

pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new() -> HittableList {
    return HittableList { objects: Vec::new() };
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<HitRecord> {
    let mut closest_hit: Option<HitRecord> = None;

    for object in &self.objects {
      match closest_hit.clone() {
        Some(c_hit) => {
          match object.hit(ray, t_min, t_max) {
            Some(o_hit) => {
              if o_hit.t < c_hit.t {
                closest_hit = Some(o_hit);
              }
            },
            None => ()
          }
        },
        None => closest_hit = object.hit(ray, t_min, t_max)
      }
    }

    return closest_hit;
  }
}