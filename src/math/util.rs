use std::ops::Range;

use crate::color::Color;
use rand::Rng;

pub fn lerp(t: f32, min: &Color, max: &Color) -> Color {
  return *min * (1.0 - t) + *max * t;
}

pub fn clamp(val: &f32, min: f32, max: f32) -> f32 {
  if *val < min { return min };
  if *val > max { return max };
  return *val;
}

pub fn random_float() -> f32 {
  return rand::thread_rng().gen::<f32>()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
  return rand::thread_rng().gen_range::<f32, Range<f32>>(Range { start: min, end: max });
}