extern crate avow;
extern crate yaecs;

use std::any::TypeId;

use yaecs::entity::Entity;
use yaecs::global::Globals;
use yaecs::system::System;
use yaecs::world::World;

#[test]
fn world_starts_empty() {
  let world = World::new();
  assert!(world.entities.is_empty());
  assert!(world.systems.is_empty());
}

#[test]
fn can_create_an_entity_with_no_components() {
  let mut world = World::new();
  assert!(world.entities.is_empty());
  world.create_entity(| _ | ());
  assert!(!world.entities.is_empty());
}

#[test]
fn can_create_an_entity_with_multiple_components() {
  let mut world = World::new();
  assert!(world.entities.is_empty());
  world.create_entity(move | components | {
    components.insert(FakeComponent(128));
    components.insert(OtherFakeComponent {});
  });
  assert!(!world.entities.is_empty());
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
  world.create_entity(| components | {
    components.insert(FakeComponent(128));
    components.insert(OtherFakeComponent {});
  });
  world.update();
}

struct FakeComponent(u8);

struct OtherFakeComponent;

struct FakeSystem;

impl FakeSystem {
  fn new() -> FakeSystem {
    FakeSystem { }
  }
}

impl System for FakeSystem {
  fn process(&mut self, entities: Vec<&Entity>, globals: &Globals) { }
  fn operates_on(&self) -> Vec<TypeId> {
    vec!(TypeId::of::<FakeComponent>())
  }
}
