#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl std::ops::Index<i32> for Vector3 {
  type Output = f64;
  fn index(&self, index: i32) -> &Self::Output {
    match index {
      0 => &(self.x),
      1 => &(self.y),
      2 => &(self.z),
      _ => panic!("Invalid index for Vector3: {}", index)
    }
  }
}
impl std::ops::IndexMut<i32> for Vector3 {
  fn index_mut(&mut self, index: i32) -> &mut Self::Output {
    match index {
      0 => &mut (self.x),
      1 => &mut (self.y),
      2 => &mut (self.z),
      _ => panic!("Invalid index for Vector3: {}", index)
    }
  }
}

impl std::ops::Add<Vector3> for Vector3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z) } 
  }
}
impl std::ops::AddAssign<Vector3> for Vector3 { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl std::ops::Sub<Vector3> for Vector3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z) } 
  }
}
impl std::ops::SubAssign<Vector3> for Vector3 { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl std::ops::Mul<f64> for Vector3 {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self::Output {
    Self { x: (self.x * rhs), y: (self.y * rhs), z: (self.z * rhs) }
  }
}
impl std::ops::MulAssign<f64> for Vector3 { fn mul_assign(&mut self, rhs: f64) { *self = *self * rhs; } }
impl std::ops::Div<f64> for Vector3 { 
  type Output = Self;
  fn div(self, rhs: f64) -> Self::Output { self * (1.0/rhs) }
}
impl std::ops::DivAssign<f64> for Vector3 { fn div_assign(&mut self, rhs: f64) { *self = *self / rhs; } }
impl std::ops::Mul<Vector3> for Vector3 {
  type Output = f64;
  fn mul(self, rhs: Vector3) -> Self::Output {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }
}

impl Vector3 {
  pub fn origin() -> Self { Vector3 { x: 0.0, y: 0.0, z: 0.0 } }
  pub fn new(a: f64, b: f64, c: f64) -> Self { Vector3 { x: a, y: b, z: c } }
  pub fn from_vec(v: &Vec<f64>) -> Self { Vector3 { x: v[0], y: v[1], z: v[2] } }
  pub fn magnitude(&self) -> f64 { f64::sqrt(*self * *self) }
  pub fn distance(&self, v: &Self) -> f64 { (*self - *v).magnitude() }
  pub fn unit(&self) -> Vector3 { if self.magnitude() == 0.0 {*self} else {*self / self.magnitude()}}
  pub fn normalize(&mut self) -> &Vector3 { *self = self.unit(); self }
  pub fn cross(&self, v: Self) -> Vector3 { Vector3 { x: self.y*v.z - self.z*v.y, y: self.z*v.x - self.x*v.z, z: self.x*v.y - self.y*v.x} }
}

impl std::fmt::Display for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
  }
}
