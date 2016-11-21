//! A world is the containing unit that allows the ECS to function.

use component::ComponentStore;
use entity::Entity;
use global::Globals;
use system::System;

pub struct World {
  pub entities: Vec<Entity>,
  pub globals: Globals,
  pub systems: Vec<Box<System>>
}

impl World {
  pub fn new() -> World {
    World {
      entities: vec!(),
      globals: Globals::new(),
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
    let ref globals = self.globals;
    for system in &mut self.systems {
      let ref entities = &self.entities;
      let filtered_entities = entities.into_iter()
        .filter(| e | system.operates_on().into_iter().all(| id | e.components.contains_key(id)))
        .collect();
      system.process(filtered_entities, globals);
    }
  }
}
