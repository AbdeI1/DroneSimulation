use std::collections::{HashMap, VecDeque};

use crate::{
  transit::entities::entity::{Entity, EntityTrait},
  math::vector3::Vector3, graph::graph::Graph};
use serde_json::Value;

use super::factory::{CompositeFactory, DroneFactory, RobotFactory, HumanFactory, HelicopterFactory, PackageFactory};

#[derive(Debug)]
pub struct Trip {
  drone_id: i32,
  package_id: i32,
  robot_id: i32,
  active: bool,
  finished: bool,
  current_destination: Vector3
}

pub struct SimulationModel<'a> {
  pub entities: HashMap<i32, Entity<'a>>,
  scheduler: VecDeque<i32>,
  trips: Vec<Trip>,
  factory: CompositeFactory,
  graph: Graph
}

fn get_nearest_entity(entities: &HashMap<i32, Vector3>, e: Vector3) -> Option<(i32, Vector3)> {
  let mut x = entities.iter()
    .map(|(i, o)| (o.distance(&e), *i, *o))
    .collect::<Vec<(f64, i32, Vector3)>>();
  x.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
  match x.get(0) {
    Some((_, entity, pos)) => Some((*entity, *pos)),
    _ => None
  }
}

impl<'a> SimulationModel<'a> {
  pub fn new() -> Self {
    let mut model = SimulationModel {
      entities: HashMap::new(),
      scheduler: VecDeque::new(),
      trips: vec![],
      factory: CompositeFactory::new(), 
      graph: Graph::new()
    };
    model.factory.add_factory(Box::new(DroneFactory {}));
    model.factory.add_factory(Box::new(RobotFactory {}));
    model.factory.add_factory(Box::new(HumanFactory {}));
    model.factory.add_factory(Box::new(HelicopterFactory {}));
    model.factory.add_factory(Box::new(PackageFactory {}));
    model
  }
  pub fn set_graph(&mut self, graph: Graph) {
    self.graph = graph;
  }
  pub fn create_entity<'b>(&'a mut self, data: Value) -> Option<&Entity<'b>> where 'a: 'b {
    let entity_name = data["name"].to_string();
    let p: Vec<f64> = data["position"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let entity_pos = Vector3::new(p[0], p[1], p[2]);
    println!("{}: {}", entity_name, entity_pos);
    let ret = self.factory.create_any_entity(&data);
    if let Some(entity ) = ret {
      let id = entity.get_id();
      entity.link_model(self);
      self.entities.insert(id, entity);
      self.entities.get(&id)
    } else {
      None
    }
  }
  pub fn schedule_trip(&mut self, data: &Value) -> Option<Value> {
    // let name = data["name"].to_string();
    // let start = data["start"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect::<Vec<f64>>();
    // let end = data["end"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect::<Vec<f64>>();
    // for (_, entity) in self.entities.iter_mut() {
    //   if let Entity::Package(package) = entity {
    //     if package.get_availability() && package.get_details()["name"].to_string() == name {
    //       // robot.set_destination(Vector3::from_vec(&end));
    //       // robot.set_strategy(data["search"].as_str().unwrap().to_string());
    //       // self.scheduler.insert(robot.get_id());
    //       // println!("{}: {:?} --> {:?}", name, start, end);
    //     }
    //   }
    // }
    Some(data.clone())
  }
  pub fn stop(&mut self) {
    
  }
  pub fn update(&mut self, dt: f64) {
    // self.create_trips();
    // self.update_human_movements();
    self.update_all_entities(dt);
    // self.update_trips();
  }
  // fn create_trips(&mut self) {
  //   let mut distances = self.scheduler.iter()
  //     .map(|k| (k, &self.entities[k]))
  //     .map(|(k, v)| (*k, v.get_position()))
  //     .collect::<HashMap<i32, Vector3>>();
  //   let mut scheduled = HashSet::new();
  //   for (_, entity) in self.entities.iter_mut() {
  //     match entity {
  //       Entity::Drone(d) => {
  //         if d.get_availability() && self.scheduler.len() > 0 {
  //           match get_nearest_entity(&distances, d.get_position()) {
  //             Some((e, p)) => {
  //               let t = Trip {
  //                 carrier_id: d.get_id(),
  //                 passenger_id: e,
  //                 active: false,
  //                 finished: false,
  //                 current_destination: p
  //               };
  //               self.trips.push(t);
  //               d.establish_trip(p);
  //               scheduled.insert(e);
  //               distances.remove(&e);
  //               self.scheduler.remove(&e);
  //             },
  //             _ => ()
  //           };
  //         }
  //       },
  //       _ => ()
  //     }
  //   }
  //   for id in scheduled {
  //     self.entities.get_mut(&id).unwrap().set_availability(false);
  //   }
  // }
  // fn update_human_movements(&mut self) {
  //   for (_, entity) in self.entities.iter_mut() {
  //     match entity {
  //       Entity::Human(h) => h.set_movement(&self.graph),
  //       _ => ()
  //     }
  //   }
  // }
  fn update_all_entities(&mut self, dt: f64) {
    for (_, entity) in self.entities.iter_mut() {
      entity.update(dt);
    }
  }
  // fn update_trips(&mut self) {
  //   for trip in self.trips.iter_mut() {
  //     let (pos1, dir1) = {
  //       let x= self.entities.get(&trip.carrier_id).unwrap();
  //       (x.get_position(), x.get_direction())
  //     };
  //     let trip_info = match self.entities.get(&trip.passenger_id) {
  //       Some(Entity::Robot(r)) => Some((r.get_strategy(), r.get_destination())),
  //       _ => None
  //     };
  //     if let Entity::Drone(d) = self.entities.get_mut(&trip.carrier_id).unwrap() {
  //       if (trip.active && d.to_final_destination.is_none()) || trip_info.is_none() {
  //         d.finish_trip();
  //         trip.finished = true;
  //       } else if !trip.active && d.to_robot.is_none() {
  //         let (strat, dest) = trip_info.unwrap();
  //         d.continue_trip(strat, dest, &self.graph);
  //         trip.current_destination = dest;
  //         trip.active = true;
  //       }
  //     }
  //     if trip.active {
  //       let e2 = self.entities.get_mut(&trip.passenger_id).unwrap();
  //       e2.set_position(pos1);
  //       e2.set_direction(dir1);
  //     }
  //   }
  //   self.trips.retain(|t| !t.finished);
  // }
}
