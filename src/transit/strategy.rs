use crate::math::vector3::Vector3;

#[derive(Debug)]
pub struct MovementInfo {
  pub position: Vector3,
  pub direction: Vector3,
  pub speed: f64
}

pub trait MovementStrategy {
  fn move_entity(&mut self, entity: MovementInfo, dt: f64) -> (Vector3, Vector3);
  fn is_completed(&self) -> bool;
}

pub struct PathStrategy {
  path: Vec<Vector3>,
  index: usize
}

impl PathStrategy {
  pub fn new() -> Self {
    PathStrategy { path: vec![], index: 0 }
  }
  pub fn from_path(path: Vec<Vector3>) -> Self {
    PathStrategy {
      index: 0,
      path
    }
  }
  pub fn from_start_end(start: Vector3, end: Vector3) -> Self {
    PathStrategy { 
      index: 0,
      path: vec![start, end]
    }
  }
}

impl MovementStrategy for PathStrategy {
  fn move_entity(&mut self, entity: MovementInfo, dt: f64) -> (Vector3, Vector3) {
    if self.is_completed() { return (entity.position, entity.direction); }
    let vi = self.path[self.index];
    let dir = (vi - entity.position).unit();
    let pos = entity.position + dir*entity.speed*dt;
    if vi.distance(&pos) < 4. {
      self.index += 1;
    };
    (pos, dir)
  }
  fn is_completed(&self) -> bool {
    self.index >= self.path.len()
  }
}

pub struct SpinDecorator<T: MovementStrategy> {
  strat: T,
  time: f64
}
impl <T: MovementStrategy> SpinDecorator<T> {
  pub fn new(strat: T, time: f64) -> Self {
    SpinDecorator { strat, time }
  }
}
impl <T: MovementStrategy> MovementStrategy for SpinDecorator<T> {
  fn move_entity(&mut self, entity: MovementInfo, dt: f64) -> (Vector3, Vector3) {
    if self.strat.is_completed() {
      let angle = entity.speed*dt;
      let dir = entity.direction;
      let mut new_dir = Vector3::origin();
      new_dir.x = dir.x * f64::cos(angle) - dir.z * f64::sin(angle);
      new_dir.y = dir.y;
      new_dir.z = dir.x * f64::sin(angle) + dir.z * f64::cos(angle);
      self.time -= dt;
      (entity.position, new_dir)
    } else { self.strat.move_entity(entity, dt) }
  }
  fn is_completed(&self) -> bool {
    self.time <= 0.
  }
}

pub struct JumpDecorator<T: MovementStrategy> {
  strat: T,
  time: f64,
  height: f64,
  going_up: bool,
  y_level: f64
}
impl <T: MovementStrategy> JumpDecorator<T> {
  pub fn new(strat: T, time: f64, height: f64) -> Self {
    JumpDecorator { strat, time, height,
      going_up: true,
      y_level: 0. 
    }
  }
}
impl <T: MovementStrategy> MovementStrategy for JumpDecorator<T> {
  fn move_entity(&mut self, entity: MovementInfo, dt: f64) -> (Vector3, Vector3) {
    if self.strat.is_completed() {
      let mut final_pos = entity.position;
      if self.going_up {
        final_pos.y += dt*entity.speed;
        self.y_level += dt*entity.speed;
        if self.y_level >= self.height {
          self.going_up = false;
        }
      } else {
        final_pos.y -= dt*entity.speed;
        self.y_level -= dt*entity.speed;
        if self.y_level <= 0. {
          self.going_up = true;
        }
      }
      self.time -= dt;
      (final_pos, entity.direction)
    } else { self.strat.move_entity(entity, dt) }
  }
  fn is_completed(&self) -> bool {
    self.time <= 0.
  }
}
