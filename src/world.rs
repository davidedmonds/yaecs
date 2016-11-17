//! A world is the containing unit that allows the ECS to function.

use component::Component;
use stash::Stash;
use std::collections::HashSet;
use system::System;

pub struct World {
  entities: Stash<HashSet<Component>>,
  systems: Vec<Box<System>>
}

impl World {
  pub fn new() -> World {
    World {
      entities: Stash::new(),
      systems: vec!()
    }
  }

  pub fn create_entity<CB>(&mut self, cb: CB) -> usize
    where CB: Fn(&HashSet<Component>) -> () {
    let entity = self.entities.put(HashSet::new());
    cb(&self.entities.get(entity).unwrap());
    entity
  }

  pub fn register(&mut self, system: Box<System>) {
    self.systems.push(system);
  }

  pub fn update(&mut self) {
    for system in &mut self.systems {
      system.process(&self.entities);
    }
  }
}

#[cfg(test)]
mod tests {
  extern crate avow;

  use super::*;

  use component::Component;
  use stash::Stash;
  use std::collections::HashSet;
  use std::mem;
  use system::System;

  #[test]
  fn world_starts_empty() {
    let world = World::new();
    assert!(world.entities.is_empty());
    assert!(world.systems.is_empty());
  }

  #[test]
  fn can_create_an_entity() {
    let mut world = World::new();
    assert!(world.entities.is_empty());
    let entity = world.create_entity(| _ | ());
    assert!(!world.entities.is_empty());
    assert_eq!(entity, 0);
  }

  #[test]
  fn can_register_a_system() {
    let mut world = World::new();
    assert!(world.systems.is_empty());
    world.register(Box::new(FakeSystem::new()));
    assert!(!world.systems.is_empty());
  }

  #[test]
  fn systems_are_called_during_update() {
    let mut world = World::new();
    world.register(Box::new(FakeSystem::new()));
    world.update();
  }

  struct FakeSystem {
    called: bool
  }

  impl FakeSystem {
    fn new() -> FakeSystem {
      FakeSystem { called: false }
    }
  }

  impl System for FakeSystem {
    fn process(&mut self, _: &Stash<HashSet<Component>>) {
    }
  }
}
