use std::sync::Arc;

use super::entity::{EntityTrait, EntityStruct};
use crate::graph::graph::Graph;
use crate::graph::routing::{AStar, DepthFirstSearch, Dijkstras};
use crate::math::vector3::Vector3;
use crate::transit::simulation_model::SimulationModel;
use crate::transit::strategy::{MovementStrategy, PathStrategy, SpinDecorator, JumpDecorator};
use serde_json::Value;


pub struct Drone {
  entity_info: EntityStruct,
  availability: bool,
  pub to_robot: Option<Box<dyn MovementStrategy>>,
  pub to_final_destination: Option<Box<dyn MovementStrategy>>,
}

unsafe impl Send for Drone {}
unsafe impl Sync for Drone {}

impl Drone {
  pub fn new(id: i32, data: &Value) -> Self {
    Drone {
      entity_info: EntityStruct::new(id, data),
      availability: true,
      to_robot: None,
      to_final_destination: None
    }
  }
  pub fn establish_trip(&mut self, dest: Vector3) {
    self.to_robot = Some(Box::new(PathStrategy::from_start_end(self.get_position(), dest)));
    self.availability = false;
  }
  pub fn continue_trip(&mut self, strat: String, dest: Vector3, graph: &Graph) {
    // self.destination = dest;
    self.to_final_destination = match strat.as_str() {
      "astar" => Some(Box::new(
        JumpDecorator::new(
          PathStrategy::from_path(
            graph.get_path(self.get_position(), dest, 
              Box::new(AStar::new())
            ).unwrap()
          )
        , 4., 10.)
      )),
      "dfs" => Some(Box::new(
        SpinDecorator::new(
          JumpDecorator::new(
            PathStrategy::from_path(
              graph.get_path(self.get_position(), dest, 
                Box::new(DepthFirstSearch::new())
              ).unwrap()
            )
          , 4., 10.)
        , 4.)
      )),
      "dijkstra" => Some(Box::new(
        JumpDecorator::new(
          SpinDecorator::new(
            PathStrategy::from_path(
              graph.get_path(self.get_position(), dest, 
                Box::new(Dijkstras::new())
              ).unwrap()
            )
          , 4.)
        , 4., 10.)
      )),
      _ => Some(Box::new(PathStrategy::from_start_end(self.get_position(), dest)))
    };
  }
  pub fn finish_trip(&mut self) {
    self.availability = true;
  }
}

impl EntityTrait for Drone {
  fn get_id(&self) -> i32 { self.entity_info.id }
  fn get_position(&self) -> Vector3 { self.entity_info.position }
  fn get_direction(&self) -> Vector3 { self.entity_info.direction }
  fn get_speed(&self) -> f64 { self.entity_info.speed }
  fn get_details(&self) -> &Value { &self.entity_info.details }
  fn update(&mut self, dt: f64) {
    let mi = self.get_movement_info();
    if let Some(strat) = &mut self.to_robot {
      (self.entity_info.position, self.entity_info.direction) = strat.move_entity(mi, dt);
      if strat.is_completed() {
        self.to_robot = None;
      }
    } else if let Some(strat) = &mut self.to_final_destination {
      (self.entity_info.position, self.entity_info.direction) = strat.move_entity(mi, dt);
      if strat.is_completed() {
        self.to_final_destination = None;
      }
    };
  }
  fn set_position(&mut self, pos: Vector3) { self.entity_info.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.entity_info.direction = dir; }
  fn link_graph(&mut self, graph: Arc<Graph>) { self.entity_info.model = Some(graph); }
}
