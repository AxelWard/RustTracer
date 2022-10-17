use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Vec3 {
  pub fn length(&self) -> f32 {
    return self.x * self.x + self.y * self.y + self.z * self.z;
  }

  pub fn length_squared(&self) -> f32 {
    let len = self.length();
    return len * len;
  }

  pub fn unit(&self) -> Vec3 {
    return *self / self.length();
  }

  pub fn dot(&self, v2: &Vec3) -> f32 {
    return self.x * v2.x + self.y * v2.y + self.z * v2.z;
  }
  
  pub fn cross(&self, v2: &Vec3) -> Vec3 {
    return Vec3 {
      x: self.y * v2.z - self.z * v2.y,
      y: self.z * v2.x - self.x * v2.z,
      z: self.x * v2.y - self.y * v2.x
    }
  }
}

impl ops::Mul<f32> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f32) -> Self::Output {
    return Vec3 {
      z: self.x * rhs,
      y: self.y * rhs,
      x: self.z * rhs
    };
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    return Vec3 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z
    };
  }
}

impl ops::Div<f32> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f32) -> Self::Output {
    return Vec3 {
      z: self.x / rhs,
      y: self.y / rhs,
      x: self.z / rhs
    };
  }
}

impl ops::Div<Vec3> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: Vec3) -> Self::Output {
    return Vec3 {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z
    };
  }
}

impl ops::Add<f32> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: f32) -> Self::Output {
    return Vec3 {
      x: self.x + rhs,
      y: self.y + rhs,
      z: self.z + rhs
    };
  }
}

impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Self::Output {
    return Vec3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z
    };
  }
}

impl ops::Sub<f32> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: f32) -> Self::Output {
    return Vec3 {
      x: self.x - rhs,
      y: self.y - rhs,
      z: self.z - rhs
    };
  }
}

impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Self::Output {
    return Vec3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z
    };
  }
}