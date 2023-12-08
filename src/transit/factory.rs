use serde_json::Value;
use super::entities::{
  entity::Entity,
  drone::Drone,
  robot::Robot,
  human::Human,
  helicopter::Helicopter,
  package::Package
};

pub trait EntityFactory {
  fn create_entity<'a>(&self, id: i32, data: &Value) -> Option<Entity<'a>>;
}

pub struct DroneFactory {}
impl EntityFactory for DroneFactory {
  fn create_entity<'a>(&self, id: i32, data: &Value) -> Option<Entity<'a>> {
    match data["type"].as_str().unwrap() {
      "drone" => Some(Entity::Drone(Drone::new(id, data))),
      _ => None
    }
  }
}

pub struct RobotFactory {}
impl EntityFactory for RobotFactory {
  fn create_entity<'a>(&self, id: i32, data: &Value) -> Option<Entity<'a>> {
    match data["type"].as_str().unwrap() {
      "robot" => Some(Entity::Robot(Robot::new(id, data))),
      _ => None
    }
  }
}

pub struct HumanFactory {}
impl EntityFactory for HumanFactory {
  fn create_entity<'a>(&self, id: i32, data: &Value) -> Option<Entity<'a>> {
    match data["type"].as_str().unwrap() {
      "human" => Some(Entity::Human(Human::new(id, data))),
      _ => None
    }
  }
}

pub struct HelicopterFactory {}
impl EntityFactory for HelicopterFactory {
  fn create_entity<'a>(&self, id: i32, data: &Value) -> Option<Entity<'a>> {
    match data["type"].as_str().unwrap() {
      "helicopter" => Some(Entity::Helicopter(Helicopter::new(id, data))),
      _ => None
    }
  }
}

pub struct PackageFactory {}
impl EntityFactory for PackageFactory {
  fn create_entity<'a>(&self, id: i32, data: &Value) -> Option<Entity<'a>> {
    match data["type"].as_str().unwrap() {
      "package" => Some(Entity::Package(Package::new(id, data))),
      _ => None
    }
  }
}

pub struct CompositeFactory {
  id: i32,
  factories: Vec<Box<dyn EntityFactory + Send + Sync>>
}
impl CompositeFactory {
  pub fn new() -> Self {
    CompositeFactory { id: 0, factories: vec![] }
  }
  pub fn add_factory(&mut self, factory: Box<dyn EntityFactory + Send + Sync>) {
    self.factories.push(factory);
  }
  pub fn create_any_entity<'a>(&mut self, data: &Value) -> Option<Entity<'a>> {
    self.id += 1;
    self.create_entity(self.id, data)
  }
}
impl EntityFactory for CompositeFactory {
  fn create_entity<'a>(&self, id: i32, data: &Value) -> Option<Entity<'a>> {
    for factory in self.factories.iter() {
      if let Some(e) = factory.create_entity(id, data) {
        return Some(e);
      }
    };
    None
  }
}
