use std::time::SystemTime;
use camera::Camera;
use hittable::{Hittable, HittableList};
use hittable::sphere::Sphere;
use math::ray::Ray;
use math::util::random_in_unit_sphere;
use math::{util, Point};
use math::vec3::Vec3;
use time::OffsetDateTime;

use color::Color;
use file_writer::FileWriter;

mod file_writer;
mod color;
mod math;
mod camera;
mod hittable;

const SAMPLES_PER_PIXEL: i32 = 25;
const MAX_BOUNCES: u32 = 25;
const GAMMA_SCALE: f32 = 2.0;

fn main() {
  let width = 540;
  let height = 360;

  let mut writer = init_image_file(width, height);

  let start = SystemTime::now();
  let output = run(width, height);
  let duration = SystemTime::now().duration_since(start).unwrap().as_millis();

  println!("Run took {}s, outputting image.", duration as f32 / 1000.0);

  writer.write(&output);

  println!("Success!")
}

fn init_image_file(width: u16, height: u16) -> FileWriter {
  let mut writer: FileWriter;

  let time = SystemTime::now();
  let dt: OffsetDateTime = time.into();
  let formatted = dt.date().to_string() + "_" + &dt.hour().to_string() + "-" + &dt.minute().to_string() + "-" + &dt.second().to_string();

  match file_writer::open(&("./output_files/output_".to_owned() + &formatted + ".ppm")) {
      Some(w) => {
        writer = w;
        writer.clear();
      },
      None => panic!("Could not open file for output!"),
  }

  writer.write_ln("P3");
  writer.write_ln(&format!("{} {}", width, height));
  writer.write_ln("255");

  return writer;
}

fn run(width: u16, height: u16) -> String {
  let size: u32 = width as u32 * height as u32;
  let mut iter: u32 = 0;
  let mut color_string: String = String::new();

  let cam = Camera {
    position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    width: width as u32,
    height: height as u32,
    focal_length: 1.0
  };

  let mut world: HittableList = HittableList::new();
  world.add(Box::new(Sphere {
    center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
    radius: 0.5,
  }));
  world.add(Box::new(Sphere {
    center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
    radius: 100.0,
  }));

  for i in (0..height).rev() {
    for j in 0..width {
      iter += 1;

      let mut color = Color { r: 0.0, g: 0.0, b: 0.0 };

      for _ in 0..SAMPLES_PER_PIXEL {
          let u: f32 = (j as f32 + util::random_float()) / (width - 1) as f32;
          let v: f32 = (i as f32 + util::random_float()) / (height - 1) as f32;
    
          color += ray_color(&cam.get_ray(u, v), &world, MAX_BOUNCES);
      }
      
      let avg_color: Color = Color { 
        r: util::clamp(&(color.r / SAMPLES_PER_PIXEL as f32), 0.0, 1.0).sqrt(),
        g: util::clamp(&(color.g / SAMPLES_PER_PIXEL as f32), 0.0, 1.0).sqrt(),
        b: util::clamp(&(color.b / SAMPLES_PER_PIXEL as f32), 0.0, 1.0).sqrt()
      };

      color_string += &((avg_color * 255.0).to_string() + "\n");

      let percent = iter as f32 / size as f32;
      if iter % (size / 100) == 0 {
        println!("{}% - {}/{}", (percent * 100.0).round(), iter, size);
      }
    }
  }

  return color_string;
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Color {
  if depth == 0 {
    return Color { r: 0.0, g: 0.0, b: 0.0 };
  }

  match world.hit(ray, &0.0, &f32::INFINITY) {
    Some(hit) => {
      let target: Point = hit.p + hit.normal + random_in_unit_sphere();
      return ray_color(&Ray { origin: hit.p, direction: target - hit.p }, world, depth - 1) * 0.5;
    },
    None => {
      let unit_direction = ray.direction.unit();
      let t: f32 = 0.5 * (unit_direction.y + 1.0);
      return util::lerp(t, &Color { r: 1.0, g: 1.0, b: 1.0 }, &Color { r: 0.207, g: 0.737, b: 0.953 });
    }
  }
}