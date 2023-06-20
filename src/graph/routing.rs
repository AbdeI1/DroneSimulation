use std::{collections::{BinaryHeap, HashSet, HashMap}, cmp::Ordering};

use super::graph::{Graph, GraphNode};

#[derive(PartialEq, Debug)]
struct F64(f64);
impl Eq for F64 {}
impl PartialOrd for F64 {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.0 < other.0 {
      return Some(Ordering::Greater)
    } else if self.0 > other.0 {
      return  Some(Ordering::Less);
    } else {
      return Some(Ordering::Equal);
    }
  }
}
impl Ord for F64 {
  fn cmp(&self, other: &Self) -> Ordering {
    if let Some(ordering) = other.partial_cmp(self) {
      ordering
    } else {
      Ordering::Less
    }
  }
}

pub trait SearchStrategy {
  fn get_path(&self, g: &Graph, start: i32, end: i32) -> Option<Vec<i32>>;
}

pub struct DepthFirstSearch {}
impl DepthFirstSearch {
  pub fn new() -> Self {
    DepthFirstSearch {  }
  }
}
impl SearchStrategy for DepthFirstSearch {
  fn get_path(&self, g: &Graph, start: i32, end: i32) -> Option<Vec<i32>> {
    let mut s: Vec<(i32, i32)> = vec![];
    let mut v: HashSet<i32> = HashSet::new();
    let mut parents: HashMap<i32, i32> = HashMap::new();
    s.push((start, -1));
    while !s.is_empty() {
      let (n, p) = s.pop().unwrap();
      if v.contains(&n) { continue; }
      v.insert(n);
      parents.insert(n, p);
      if n == end { break; }
      for o in &g.adjacency_list[n as usize] {
        s.push((*o, n));
      }
    };
    let mut n = end;
    let mut path = vec![];
    while n != -1 {
      path.push(n);
      n = match parents.get(&n) {
        Some(v) => *v,
        _ => { return None; }
      };
    }
    path.reverse();
    Some(path)
  }
}

pub struct Dijkstras {}
impl Dijkstras {
  pub fn new() -> Self {
    Dijkstras {  }
  }
}
impl SearchStrategy for Dijkstras {
  fn get_path(&self, g: &Graph, start: i32, end: i32) -> Option<Vec<i32>> {
    let mut q: BinaryHeap<(F64, (i32, i32))> = BinaryHeap::new();
    let mut v: HashSet<i32> = HashSet::new();
    let mut parents: HashMap<i32, i32> = HashMap::new();
    q.push((F64(0.), (start, -1)));
    while !q.is_empty() {
      let (F64(d), (n, p)) = q.pop().unwrap();
      if v.contains(&n) { continue; }
      v.insert(n);
      parents.insert(n, p);
      if n == end { break; }
      let n1 = &g.nodes[n as usize];
      for o in &g.adjacency_list[n as usize] {
        let n2 = &g.nodes[*o as usize];
        let dist = n1.get_position().distance(&n2.get_position());
        q.push((F64(d + dist), (*o, n)));
      }
    };
    let mut n = end;
    let mut path = vec![];
    while n != -1 {
      path.push(n);
      n = match parents.get(&n) {
        Some(v) => *v,
        _ => { return None; }
      };
    }
    path.reverse();
    Some(path)
  }
}

pub struct AStar {
  heuristic: fn(&GraphNode, &GraphNode) -> f64
}
impl AStar {
  pub fn new() -> Self {
    AStar { 
      heuristic: |n, end| n.get_position().distance(&end.get_position())
    }
  }
  pub fn zero() -> Self {
    AStar { 
      heuristic: |_, _| 0.
    }
  }
  pub fn from(f: fn(&GraphNode, &GraphNode) -> f64) -> Self {
    AStar { heuristic: f }
  }
}
impl SearchStrategy for AStar {
  fn get_path(&self, g: &Graph, start: i32, end: i32) -> Option<Vec<i32>> {
    let mut q: BinaryHeap<(F64, (i32, i32, F64))> = BinaryHeap::new();
    let mut v: HashSet<i32> = HashSet::new();
    let mut parents: HashMap<i32, i32> = HashMap::new();
    q.push((F64(0.), (start, -1, F64(0.))));
    while !q.is_empty() {
      let (_, (n, p, F64(d))) = q.pop().unwrap();
      if v.contains(&n) { continue; }
      v.insert(n);
      parents.insert(n, p);
      if n == end { break; }
      let n1 = &g.nodes[n as usize];
      for o in &g.adjacency_list[n as usize] {
        let n2 = &g.nodes[*o as usize];
        let dist = n1.get_position().distance(&n2.get_position());
        q.push((F64(d + dist + (self.heuristic)(n2, &g.nodes[end as usize])), (*o, n, F64(d + dist))));
      }
    };
    let mut n = end;
    let mut path = vec![];
    while n != -1 {
      path.push(n);
      n = match parents.get(&n) {
        Some(v) => *v,
        _ => { return None; }
      };
    }
    path.reverse();
    Some(path)
  }
}

