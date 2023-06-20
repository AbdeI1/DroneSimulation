use std::fs;

use crate::math::vector3::Vector3;

use super::graph::Graph;

pub fn obj_graph_parser(file: String) -> Graph {
  let mut g = Graph::new();
  g.add_node(Vector3::new(-1000., -1000., -1000.));
  let f = fs::read_to_string(file).unwrap();
  let lines = f.split("\n")
    .map(|s| s.split(" ").collect::<Vec<&str>>())
    .collect::<Vec<Vec<&str>>>();
  for l in lines {
    match l[0] {
      "v" =>
        g.add_node(Vector3::from_vec(&l[1..4].iter()
          .map(|s| s.parse::<f64>().unwrap())
          .collect::<Vec<f64>>())),
      "l" => {
        let (v1, v2) = (l[1].parse::<i32>().unwrap(), l[2].parse::<i32>().unwrap());
        g.add_edge(v1, v2);
        g.add_edge(v2, v1);
      }
      _ => ()
    }
  };
  g
}