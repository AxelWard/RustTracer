use std::time::SystemTime;
use time::OffsetDateTime;
use time::macros;

use color::Color;
use file_writer::FileWriter;

mod file_writer;
mod color;
mod math;

fn main() {
  let width = 960;
  let height = 540;

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
  let mut color_string = String::new();

  for i in 0..height {
    for j in 0..width {

      let x = (j + 1) as f32;
      let y = (i + 1) as f32;
      let w = width as f32;
      let h = height as f32;

      let color = Color {
        r: (x / w) * 254.999,
        g: 255.0 - (y / h) * 254.999,
        b: ((x / w) * 254.999 * 0.5) + ((y / h) * 254.999 * 0.5)
      };

      color_string += &(color.to_string() + "\n");

      let done = (i as f32 * w + x) as u32;
      let percent = done as f32 / size as f32;
      if done % ((width as u32 * height as u32) / 100)== 0 {
        println!("{}% - {}/{}", (percent * 100.00).round(), done, size);
      }
    }
  }

  return color_string;
}