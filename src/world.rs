//! A world is the containing unit that allows the ECS to function.

use component::ComponentStore;
use entity::Entity;
use system::System;

pub struct World {
  pub entities: Vec<Entity>,
  pub systems: Vec<Box<System>>
}

impl World {
  pub fn new() -> World {
    World {
      entities: vec!(),
      systems: vec!()
    }
  }

  pub fn create_entity<CB>(&mut self, cb: CB) where CB: Fn(&mut ComponentStore) -> () {
    let mut entity = Entity::new();
    cb(&mut entity.components);
    self.entities.push(entity);
  }

  pub fn register(&mut self, system: Box<System>) {
    self.systems.push(system);
  }

  pub fn update(&mut self) {
    for system in &mut self.systems {
      system.process();
    }
  }
}
