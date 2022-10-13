use std::ops;

pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32
}

impl ToString for Color {
  fn to_string(&self) -> String {
    let red: i16 = self.r.round() as i16;
    let green: i16 = self.g.round() as i16;
    let blue: i16 = self.b.round() as i16;
  
    return format!("{} {} {}", red, green, blue);
  }
}

impl ops::Mul<f32> for Color {
  type Output = Color;

  fn mul(self, rhs: f32) -> Self::Output {
    return Color {
      r: self.r * rhs,
      g: self.g * rhs,
      b: self.b * rhs
    };
  }
}

impl ops::Mul<Color> for Color {
  type Output = Color;

  fn mul(self, rhs: Color) -> Self::Output {
    return Color {
      r: self.r * rhs.r,
      g: self.g * rhs.g,
      b: self.b * rhs.b
    };
  }
}

impl ops::Add<f32> for Color {
  type Output = Color;

  fn add(self, rhs: f32) -> Self::Output {
    return Color {
      r: self.r + rhs,
      g: self.g + rhs,
      b: self.b + rhs
    };
  }
}

impl ops::Add<Color> for Color {
  type Output = Color;

  fn add(self, rhs: Color) -> Self::Output {
    return Color {
      r: self.r + rhs.r,
      g: self.g + rhs.g,
      b: self.b + rhs.b
    };
  }
}