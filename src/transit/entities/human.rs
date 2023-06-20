use rand::prelude::*;

use super::entity::EntityTrait;
use super::super::strategy::{MovementStrategy, PathStrategy};
use crate::graph::graph::Graph;
use crate::graph::routing::AStar;
use crate::math::vector3::Vector3;
use serde_json::Value;

pub struct Human {
  id: i32,
  details: Value,
  position: Vector3,
  direction: Vector3,
  destination: Vector3,
  speed: f64,
  movement: Option<Box<dyn MovementStrategy>>
}

unsafe impl Send for Human {}
unsafe impl Sync for Human {}

impl Human {
  pub fn new(id: i32, data: &Value) -> Self {
    let mut h = Human {
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
      movement: None
    };
    h.destination = h.position;
    h
  }
  fn get_random_point(&self) -> Vector3 {
    let x = random::<f64>()*2900. - 1400.;
    let y = self.position.y;
    let z = random::<f64>()*1600. - 800.;
    Vector3::new(x, y, z)
  }
  pub fn set_movement(&mut self, g: &Graph) {
    if self.movement.is_some() { return; }
    let end = self.get_random_point();
    self.movement = Some(Box::new(PathStrategy::from_path(
      g.get_path(self.get_position(), end, Box::new(AStar::new())).unwrap()
    )))
  }
}

impl EntityTrait for Human {
  fn get_id(&self) -> i32 { self.id }
  fn get_position(&self) -> Vector3 { self.position }
  fn get_direction(&self) -> Vector3 { self.direction }
  fn get_destination(&self) -> Vector3 { self.destination }
  fn get_speed(&self) -> f64 { self.speed }
  fn get_details(&self) -> &Value { &self.details }
  fn update(&mut self, dt: f64) {
    let mi = self.get_movement_info();
    if let Some(strat) = &mut self.movement {
      (self.position, self.direction) = strat.move_entity(mi, dt);
      if strat.is_completed() {
        self.movement = None;
      }
    }
  }
  fn set_position(&mut self, pos: Vector3) { self.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.direction = dir; }
}
