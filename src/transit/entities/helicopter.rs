use std::sync::Arc;

use rand::prelude::*;

use super::entity::{EntityTrait, EntityStruct};
use super::super::strategy::{MovementStrategy, PathStrategy};
use crate::graph::graph::Graph;
use crate::math::vector3::Vector3;
use crate::transit::simulation_model::SimulationModel;
use serde_json::Value;

pub struct Helicopter {
  entity_info: EntityStruct,
  movement: Box<dyn MovementStrategy>
}

unsafe impl Send for Helicopter {}
unsafe impl Sync for Helicopter {}

impl Helicopter {
  pub fn new(id: i32, data: &Value) -> Self {
    Helicopter {
      entity_info: EntityStruct::new(id, data),
      movement: Box::new(PathStrategy::new())
    }
  }
  fn get_random_point(&self) -> Vector3 {
    let x = random::<f64>()*2900. - 1400.;
    let y = self.entity_info.position.y;
    let z = random::<f64>()*1600. - 800.;
    Vector3::new(x, y, z)
  }
}

impl EntityTrait for Helicopter {
  fn get_id(&self) -> i32 { self.entity_info.id }
  fn get_position(&self) -> Vector3 { self.entity_info.position }
  fn get_direction(&self) -> Vector3 { self.entity_info.direction }
  fn get_speed(&self) -> f64 { self.entity_info.speed }
  fn get_details(&self) -> &Value { &self.entity_info.details }
  fn update(&mut self, dt: f64) {
    if self.movement.is_completed() {
      self.movement = Box::new(PathStrategy::from_start_end(self.entity_info.position, self.get_random_point()));
    }
    (self.entity_info.position, self.entity_info.direction) = self.movement.move_entity(self.get_movement_info(), dt);
  }
  fn set_position(&mut self, pos: Vector3) { self.entity_info.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.entity_info.direction = dir; }
  fn link_graph(&mut self, graph: Arc<Graph>) { self.entity_info.model = Some(graph); }
}
