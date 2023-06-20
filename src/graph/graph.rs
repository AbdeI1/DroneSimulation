use crate::math::vector3::Vector3;

use super::routing::SearchStrategy;

#[derive(Debug)]
pub struct GraphNode {
  id: i32,
  position: Vector3
}
impl GraphNode {
  pub fn new(id: i32, position: Vector3) -> Self {
    GraphNode { id, position }
  }
  pub fn get_id(&self) -> i32 { self.id }
  pub fn get_position(&self) -> Vector3 { self.position }
}

#[derive(Debug)]
pub struct Graph {
  pub adjacency_list: Vec<Vec<i32>>,
  pub nodes: Vec<GraphNode>
}
impl Graph {
  pub fn new() -> Self {
    Graph {
      adjacency_list: vec![],
      nodes: vec![]
    }
  }
  pub fn add_node(&mut self, position: Vector3) {
    self.nodes.push(GraphNode::new(self.nodes.len() as i32, position));
    self.adjacency_list.push(vec![]);
  }
  pub fn add_edge(&mut self, n1: i32, n2: i32) {
    self.adjacency_list[n1 as usize].push(n2);
  }
  pub fn nearest_node(&self, position: Vector3) -> i32 {
    let (mut min_i, mut min_d) = (-1, f64::INFINITY);
    for i in 0..self.nodes.len() {
      let d = self.nodes[i].position.distance(&position);
      if d < min_d {
        min_d = d;
        min_i = i as i32;
      }
    }
    min_i 
  }
  pub fn get_path(&self, start: Vector3, end: Vector3, strat: Box<dyn SearchStrategy>) -> Option<Vec<Vector3>> {
    let n1 = self.nearest_node(start);
    let n2 = self.nearest_node(end);
    match strat.get_path(self, n1, n2) {
      Some(p) => Some(p.iter()
        .map(|i| self.nodes[*i as usize].get_position())
        .collect()),
      None => None
    }
  }
}
