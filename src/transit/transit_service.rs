use std::{time::SystemTime, sync::{Arc, RwLock}};
use futures_util::lock::Mutex;
use serde_json::{json, Value};

use crate::{Client, graph::parsers::obj_graph_parser};
use super::simulation_model;
use super::entities::entity;

use simulation_model::SimulationModel;
use entity::{Entity, EntityTrait};

pub struct TransitServer<'a> {
  client: &'a Client,
  total_time: f64,
  start: SystemTime,
  model: SimulationModel,
}

impl<'a> TransitServer<'a> {
  pub fn new(cl: &'a Client) -> Self {
    let mut server = TransitServer {
      client: cl,
      total_time: 0.,
      start: SystemTime::now(),
      model: SimulationModel::new()
    };
    server.model.set_graph(obj_graph_parser("web/assets/model/routes.obj".to_string()));
    server
  }
  pub fn recieve_message(&mut self, message: &str) {
    let data: Value = serde_json::from_str(message).unwrap();
    if let Value::String(cmd) = &data["command"] {
      match cmd.as_str() {
        "CreateEntity" => {
          let x = self.model.create_entity(data);
          if let Some(entity) = x {
            self.send_entity_event("AddEntity", &entity);
          }
          ()
        },
        "ScheduleTrip" => if let Some(data) = self.model.schedule_trip(&data) {
          self.model.update(0.);
          self.send_event_to_view("TripScheduled", &data)
        }
        "Update" => {
          let diff = SystemTime::now().duration_since(self.start).unwrap();
          let delta = diff.as_secs_f64() - self.total_time;
          self.total_time += delta;
          let sim_speed = data["simSpeed"].as_f64().unwrap();
          let dt = delta * sim_speed;
          if dt > 0.1 {
            let mut f = 0.;
            while f < dt {
              self.model.update(0.01);
              f += 0.01;
            }
          } else { self.model.update(dt); }
          for (_, entity) in self.model.entities.iter() {
            self.send_entity_event("UpdateEntity", entity);
          }
        },
        "Stop" => {
          self.model.stop();
        }
        _ => ()
      }
    }
  }
  pub fn send_entity_event(&self, event: &str, entity: &Arc<RwLock<Entity>>) {
    entity.read().and_then(|entity_ref| {
      let pos = entity_ref.get_position();
      let dir = entity_ref.get_direction();
      let col = entity_ref.get_color();
      self.send_event_to_view(event, &json!({
        "id": entity_ref.get_id(),
        "pos": [pos.x, pos.y, pos.z],
        "dir": [dir.x, dir.y, dir.z],
        "color": match col {
          Some(c) => Value::String(c),
          None => Value::Null
        },
        "details": entity_ref.get_details()
      }));
      Ok(())
    });
    ()
  }
  pub fn remove_entity(&self, id: i32) {
    self.send_event_to_view("RemoveEntity", &json!({
      "id": id
    }))
  }
  pub fn send_event_to_view(&self, event: &str, details: &Value) {
    self.send_message(&json!({
      "event": event,
      "details": details
    }).to_string());
  }
  pub fn send_message(&self, message: &str) {
    if let Some(sink) = &self.client.sender {
      let _ = sink.send(Ok(warp::ws::Message::text(message)));
    }
  }
}
