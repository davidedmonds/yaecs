//! A world is the containing unit that allows the ECS to function.

use component::Component;
use entity::Entity;
use system::System;
use std::collections::HashMap;

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

  pub fn create_entity<CB>(&mut self, cb: CB) where CB: Fn() -> (HashMap<&'static str, Box<Component>>) {
    self.entities.push(Entity {
      components: cb()
    });
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
