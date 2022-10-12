pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl ToString for Color {

  fn to_string(&self) -> String {
    let red: i16 = self.r.round() as i16;
    let green: i16 = self.g.round() as i16;
    let blue: i16 = self.b.round() as i16;
  
    return format!("{} {} {}", red, green, blue);
  }

}