use std::sync::Arc;

use rand::prelude::*;

use super::entity::{EntityTrait, EntityStruct};
use super::super::strategy::{MovementStrategy, PathStrategy};
use crate::graph::graph::Graph;
use crate::graph::routing::AStar;
use crate::math::vector3::Vector3;
use crate::transit::simulation_model::SimulationModel;
use serde_json::Value;

pub struct Human {
  entity_info: EntityStruct,
  destination: Vector3,
  movement: Option<Box<dyn MovementStrategy>>
}

unsafe impl Send for Human {}
unsafe impl Sync for Human {}

impl Human {
  pub fn new(id: i32, data: &Value) -> Self {
    let mut h: Human = Human {
      entity_info: EntityStruct::new(id, data),
      destination: Vector3::origin(),
      movement: None
    };
    h.destination = h.entity_info.position;
    h
  }
  fn get_random_point(&self) -> Vector3 {
    let x = random::<f64>()*2900. - 1400.;
    let y = self.entity_info.position.y;
    let z = random::<f64>()*1600. - 800.;
    Vector3::new(x, y, z)
  }
  pub fn set_movement(&mut self) {
    if self.movement.is_some() { return; }
    let end = self.get_random_point();
    self.entity_info.model.clone().and_then(|graph| {
      self.movement = Some(Box::new(PathStrategy::from_path(
       graph.get_path(self.get_position(), end, Box::new(AStar::new())).unwrap()
      )));
      Some(())
    });
    ()
    
  }
}

impl EntityTrait for Human {
  fn get_id(&self) -> i32 { self.entity_info.id }
  fn get_position(&self) -> Vector3 { self.entity_info.position }
  fn get_direction(&self) -> Vector3 { self.entity_info.direction }
  fn get_speed(&self) -> f64 { self.entity_info.speed }
  fn get_details(&self) -> &Value { &self.entity_info.details }
  fn update(&mut self, dt: f64) {
    let mi = self.get_movement_info();
    if let Some(strat) = &mut self.movement {
      (self.entity_info.position, self.entity_info.direction) = strat.move_entity(mi, dt);
      if strat.is_completed() {
        self.movement = None;
      }
    } else {
      self.set_movement();
    }
  }
  fn set_position(&mut self, pos: Vector3) { self.entity_info.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.entity_info.direction = dir; }
  fn link_graph(&mut self, graph: Arc<Graph>) { self.entity_info.model = Some(graph); }
}
