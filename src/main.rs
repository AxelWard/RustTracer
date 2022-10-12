use color::{Color};
use file_writer::FileWriter;
use uuid::Uuid;

mod file_writer;
mod color;

fn main() {

  let mut writer: FileWriter;
  match file_writer::open(&("./output_files/output_".to_owned() + &Uuid::new_v4().to_string() + ".ppm")) {
      Some(w) => {
        println!("Created file!");
        writer = w;
        writer.clear();
      },
      None => panic!("Could not open file for output!"),
  }

  let width = 1920;
  let height = 1080;

  init_image_file(&mut writer, width, height);
  writer.write(&hello_image(width, height));

}

fn init_image_file(writer: &mut FileWriter, width: u16, height: u16) {
  writer.write_ln("P3");
  writer.write_ln(&format!("{} {}", width, height));
  writer.write_ln("255");
}

fn hello_image(width: u16, height: u16) -> String {
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
        b: ((x / w) * 254.999 * 0.5) + ((y / h) * 254.999 * 0.5),
        a: 0.0
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