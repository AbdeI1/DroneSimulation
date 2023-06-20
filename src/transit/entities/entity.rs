use crate::{math::vector3, transit::strategy::MovementInfo};
use serde_json::Value;
use enum_dispatch::enum_dispatch;

use vector3::Vector3;

use super::{drone::Drone, helicopter::Helicopter, robot::Robot, human::Human};

#[enum_dispatch]
pub enum Entity {
  Drone(Drone),
  Helicopter(Helicopter),
  Robot(Robot),
  Human(Human)
}

#[enum_dispatch(Entity)]
pub trait EntityTrait {
  fn get_id(&self) -> i32;
  fn get_position(&self) -> Vector3 { Vector3::origin() }
  fn get_direction(&self) -> Vector3 { Vector3::origin() }
  fn get_destination(&self) -> Vector3 { Vector3::origin() }
  fn get_color(&self) -> Option<String> { None }
  fn get_speed(&self) -> f64 { 0. }
  fn get_availability(&self) -> bool { false }
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
      destination: self.get_destination(),
      speed: self.get_speed()
    }
  }
}
