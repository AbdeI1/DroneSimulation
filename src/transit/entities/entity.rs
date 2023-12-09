use std::sync::Arc;

use crate::{math::vector3, transit::{strategy::MovementInfo, simulation_model::SimulationModel}, graph::graph::Graph};
use serde_json::Value;
use enum_dispatch::enum_dispatch;

use vector3::Vector3;

use super::{
  drone::Drone, 
  helicopter::Helicopter, 
  robot::Robot, 
  human::Human,
  package::Package
};

#[enum_dispatch]
pub enum Entity {
  Drone(Drone),
  Helicopter(Helicopter),
  Robot(Robot),
  Human(Human),
  Package(Package)
}

pub struct EntityStruct {
  pub id: i32,
  pub model: Option<Arc<Graph>>,
  pub details: Value,
  pub position: Vector3,
  pub direction: Vector3,
  pub speed: f64
}

impl EntityStruct {
  pub fn new(id: i32, data: &Value) -> Self {
    EntityStruct { 
      id, 
      model: None, 
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
    }
  }
}

#[enum_dispatch(Entity)]
pub trait EntityTrait {
  fn get_id(&self) -> i32;
  fn get_position(&self) -> Vector3 { Vector3::origin() }
  fn get_direction(&self) -> Vector3 { Vector3::origin() }
  fn get_color(&self) -> Option<String> { None }
  fn get_speed(&self) -> f64 { 0. }
  fn get_details(&self) -> &Value;
  fn update(&mut self, dt: f64);
  fn set_position(&mut self, _pos: Vector3) {}
  fn set_direction(&mut self, _dir: Vector3) {}
  fn set_destination(&mut self, _des: Vector3) {}
  fn set_availability(&mut self, _avail: bool) {}
  fn jump(&mut self, _height: f64) {}
  fn rotate(&mut self, angle: f64) {
    let dir = self.get_direction();
    let mut new_dir = Vector3::origin();
    new_dir.x = dir.x * f64::cos(angle) - dir.z * f64::sin(angle);
    new_dir.y = dir.y;
    new_dir.z = dir.x * f64::sin(angle) + dir.z * f64::cos(angle);
    self.set_direction(new_dir);
  }
  fn get_movement_info(&self) -> MovementInfo {
    MovementInfo {
      position: self.get_position(),
      direction: self.get_direction(),
      speed: self.get_speed()
    }
  }
  fn link_graph(&mut self, _graph: Arc<Graph>) {}
}
