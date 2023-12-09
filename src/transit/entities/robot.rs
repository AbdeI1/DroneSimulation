use std::sync::Arc;

use super::entity::{EntityTrait, EntityStruct};
use crate::{math::vector3::Vector3, transit::simulation_model::SimulationModel, graph::{graph::Graph, self}};
use serde_json::Value;

pub struct Robot {
  entity_info: EntityStruct,
}

unsafe impl Send for Robot {}
unsafe impl Sync for Robot {}

impl Robot {
  pub fn new(id: i32, data: &Value) -> Self {
    Robot {
      entity_info: EntityStruct::new(id, data),
    }
  }
}

impl EntityTrait for Robot {
  fn get_id(&self) -> i32 { self.entity_info.id }
  fn get_position(&self) -> Vector3 { self.entity_info.position }
  fn get_direction(&self) -> Vector3 { self.entity_info.direction }
  fn get_speed(&self) -> f64 { self.entity_info.speed }
  fn get_details(&self) -> &Value { &self.entity_info.details }
  fn update(&mut self, _dt: f64) { }
  fn set_position(&mut self, pos: Vector3) { self.entity_info.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.entity_info.direction = dir; }
  fn link_graph(&mut self, graph: Arc<Graph>) { self.entity_info.model = Some(graph); }
}
