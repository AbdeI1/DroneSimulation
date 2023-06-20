use super::entity::EntityTrait;
use crate::math::vector3::Vector3;
use serde_json::Value;

pub struct Robot {
  id: i32,
  details: Value,
  position: Vector3,
  direction: Vector3,
  destination: Vector3,
  speed: f64,
  availability: bool,
  strategy_name: String
}

unsafe impl Send for Robot {}
unsafe impl Sync for Robot {}

impl Robot {
  pub fn new(id: i32, data: &Value) -> Self {
    let mut h = Robot {
      id,
      details: data.clone(),
      speed: match data["speed"].as_f64() {
        Some(s) => s,
        _ => 10.
      },
      position: match data["position"].as_array() {
        Some(a) => match a.iter().map(|v| v.as_f64()).collect::<Vec<Option<f64>>>()[..] {
          [Some(x), Some(y), Some(z)] => Vector3::new(x, y, z),
          _ => Vector3::origin()
        },
        _ => Vector3::origin()
      },
      direction: match data["direction"].as_array() {
        Some(a) => match a.iter().map(|v| v.as_f64()).collect::<Vec<Option<f64>>>()[..] {
          [Some(x), Some(y), Some(z)] => Vector3::new(x, y, z),
          _ => Vector3::origin()
        },
        _ => Vector3::origin()
      },
      destination: Vector3::origin(),
      strategy_name: "".to_string(),
      availability: true
    };
    h.destination = h.position;
    h
  }
  pub fn set_strategy(&mut self, strat: String) { self.strategy_name = strat; }
  pub fn get_strategy(&self) -> String { self.strategy_name.clone() }
}

impl EntityTrait for Robot {
  fn get_id(&self) -> i32 { self.id }
  fn get_position(&self) -> Vector3 { self.position }
  fn get_direction(&self) -> Vector3 { self.direction }
  fn get_destination(&self) -> Vector3 { self.destination }
  fn get_availability(&self) -> bool { self.availability }
  fn get_speed(&self) -> f64 { self.speed }
  fn get_details(&self) -> &Value { &self.details }
  fn update(&mut self, _dt: f64) { }
  fn set_position(&mut self, pos: Vector3) { self.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.direction = dir; }
  fn set_destination(&mut self, des: Vector3) { self.destination = des }
  fn set_availability(&mut self, avail: bool) { self.availability = avail }
}
