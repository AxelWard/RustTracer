use crate::color::Color;

pub fn lerp(t: f32, min: &Color, max: &Color) -> Color {
  return *min * (1.0 - t) + *max * t;
}