use super::entity::EntityTrait;
use crate::graph::graph::Graph;
use crate::graph::routing::{AStar, DepthFirstSearch, Dijkstras};
use crate::math::vector3::Vector3;
use crate::transit::strategy::{MovementStrategy, PathStrategy, SpinDecorator, JumpDecorator};
use serde_json::Value;

pub struct Drone {
  id: i32,
  details: Value,
  position: Vector3,
  direction: Vector3,
  destination: Vector3,
  availability: bool,
  speed: f64,
  pub to_robot: Option<Box<dyn MovementStrategy>>,
  pub to_final_destination: Option<Box<dyn MovementStrategy>>,
}

unsafe impl Send for Drone {}
unsafe impl Sync for Drone {}

impl Drone {
  pub fn new(id: i32, data: &Value) -> Self {
    let mut h = Drone {
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
      availability: true,
      to_robot: None,
      to_final_destination: None
    };
    h.destination = h.position;
    h
  }
  pub fn establish_trip(&mut self, dest: Vector3) {
    self.to_robot = Some(Box::new(PathStrategy::from_start_end(self.get_position(), dest)));
    self.availability = false;
  }
  pub fn continue_trip(&mut self, strat: String, dest: Vector3, graph: &Graph) {
    self.destination = dest;
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
  fn get_id(&self) -> i32 { self.id }
  fn get_position(&self) -> Vector3 { self.position }
  fn get_direction(&self) -> Vector3 { self.direction }
  fn get_destination(&self) -> Vector3 { self.destination }
  fn get_availability(&self) -> bool { self.availability }
  fn get_speed(&self) -> f64 { self.speed }
  fn get_details(&self) -> &Value { &self.details }
  fn update(&mut self, dt: f64) {
    let mi = self.get_movement_info();
    if let Some(strat) = &mut self.to_robot {
      (self.position, self.direction) = strat.move_entity(mi, dt);
      if strat.is_completed() {
        self.to_robot = None;
      }
    } else if let Some(strat) = &mut self.to_final_destination {
      (self.position, self.direction) = strat.move_entity(mi, dt);
      if strat.is_completed() {
        self.to_final_destination = None;
      }
    };
  }
  fn set_position(&mut self, pos: Vector3) { self.position = pos; }
  fn set_direction(&mut self, dir: Vector3) { self.direction = dir; }
}
