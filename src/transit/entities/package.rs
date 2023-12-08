use super::entity::{EntityTrait, EntityStruct};
use crate::{math::vector3::Vector3, transit::simulation_model::SimulationModel};
use serde_json::Value;

pub struct Package<'a> {
  entity_info: EntityStruct<'a>,
}

unsafe impl Send for Package<'_> {}
unsafe impl Sync for Package<'_> {}

impl<'a> Package<'a> {
  pub fn new(id: i32, data: &Value) -> Self {
    Package {
      entity_info: EntityStruct::new(id, data)
    }
  }
}

impl<'a, 'b> EntityTrait<'b> for Package<'a> where 'b: 'a {
  fn get_id(&self) -> i32 { self.entity_info.id }
  fn get_position(&self) -> Vector3 { self.entity_info.position }
  fn get_direction(&self) -> Vector3 { self.entity_info.direction }
  fn get_speed(&self) -> f64 { self.entity_info.speed }
  fn get_details(&self) -> &Value { &self.entity_info.details }
  fn update(&mut self, _dt: f64) {}
  fn set_position(&mut self, pos: Vector3) { self.entity_info.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.entity_info.direction = dir; }
  fn link_model(&mut self, model: &'b SimulationModel<'b>) { self.entity_info.model = Some(model); }
}
