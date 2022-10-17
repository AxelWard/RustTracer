use crate::math::vec3::Vec3;

pub struct Camera {
  pub position: Vec3,
  pub width: u32,
  pub height: u32,
  pub focal_length: f32
}

impl Camera {
  pub fn aspect_ratio(&self) -> f32 {
    return self.width as f32 / self.height as f32;
  }

  pub fn viewport_height(&self) -> f32 {
    return 2.0;
  }

  pub fn viewport_width(&self) -> f32 {
    return self.aspect_ratio() * self.viewport_height();
  }

  pub fn horizontal(&self) -> Vec3 {
    return Vec3 { x: self.viewport_width(), y: 0.0, z: 0.0 };
  }

  pub fn vertical(&self) -> Vec3 {
    return Vec3 { x: 0.0, y: self.viewport_height(), z: 0.0 };
  }

  // SOMETHING HERE IS BROKEN???
  pub fn lower_left_corner(&self) -> Vec3 {
    return self.position - (self.horizontal() / 2.0) - (self.vertical() / 2.0) - Vec3 { x: 0.0, y: 0.0, z: self.focal_length };
  }
}