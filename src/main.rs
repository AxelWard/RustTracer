use std::time::SystemTime;
use camera::Camera;
use math::ray::Ray;
use math::{util, Point};
use math::vec3::Vec3;
use time::OffsetDateTime;

use color::Color;
use file_writer::FileWriter;

mod file_writer;
mod color;
mod math;
mod camera;

fn main() {
  let width = 540;
  let height = 340;

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
        println!("Created file!");
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

  println!("{}, {}", cam.viewport_width(), cam.viewport_height());
  println!("{}, {}, {}", cam.lower_left_corner().x, cam.lower_left_corner().y, cam.lower_left_corner().z);

  for i in 0..height {
    for j in 0..width {
      iter += 1;

      let u: f32 = j as f32 / (width - 1) as f32;
      let v: f32 = i as f32 / (height - 1) as f32;

      let ray: Ray = Ray {
        origin: cam.position,
        direction: cam.lower_left_corner() + (cam.horizontal() * u) + (cam.vertical() * v) - cam.position
      };

      let color: Color = ray_color(&ray);

      color_string += &(color.to_string() + "\n");

      let percent = iter as f32 / size as f32;
      if iter % (size / 100) == 0 {
        println!("{}, {} - {}, {} - {}, {}, {}", j, i, u, v, ray.direction.x, ray.direction.y, ray.direction.z);
        println!("{}% - {}/{}", (percent * 100.0).round(), iter, size);
      }
    }
  }

  return color_string;
}

fn ray_color(ray: &Ray) -> Color {
  let unit_direction = ray.direction.unit();
  let t: f32 = 0.5 * (unit_direction.y + 1.0);
  return util::lerp(t, &Color { r: 255.0, g: 255.0, b: 255.0 }, &Color { r: 127.5, g: 190.0, b: 255.0 })
}