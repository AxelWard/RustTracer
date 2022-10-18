use super::{Hittable, HitRecord};
use crate::math::{Point, vec3::Vec3, ray::Ray};

pub struct Sphere {
  pub center: Point,
  pub radius: f32
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<HitRecord> {
    let check_root = |root: &f32| -> bool {
      return root < t_min || root > t_max;
    };

    let oc: Vec3 = ray.origin - self.center;
    let a: f32 = ray.direction.length_squared();
    let half_b: f32 = oc.dot(&ray.direction);
    let c: f32 = oc.length_squared() - self.radius * self.radius;
    let discriminant: f32 = half_b * half_b - a * c;
  
    if discriminant < 0.0 { return None };

    let sqrtd: f32 = discriminant.sqrt();
    let mut root: f32 = (-half_b - sqrtd) / a;

    if check_root(&root) {
      root = (-half_b + sqrtd) / a;
      if check_root(&root) {
        return None;
      }
    }

    let hit_point = ray.at(root);
    let mut hit_record = HitRecord {
      p: hit_point,
      normal: ((hit_point - self.center) / self.radius).unit(),
      t: root,
      front_face: true
    };
    hit_record.set_front_face(ray, &((hit_point - self.center) / self.radius).unit());

    return Some(hit_record);
  }
}