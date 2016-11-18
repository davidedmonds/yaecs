extern crate avow;
extern crate yaecs;

use yaecs::component::Component;
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
  world.create_entity(| components | {
    components.add(Box::new(FakeComponent {}));
    components.add(Box::new(OtherFakeComponent {}));
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
  world.update();
}

struct FakeComponent;

impl Component for FakeComponent {
  fn id(&self) -> &'static str {
    "FakeComponent"
  }
}

struct OtherFakeComponent;

impl Component for OtherFakeComponent {
  fn id(&self) -> &'static str {
    "OtherFakeComponent"
  }
}

struct FakeSystem;

impl FakeSystem {
  fn new() -> FakeSystem {
    FakeSystem { }
  }
}

impl System for FakeSystem {
  fn process(&mut self) {
  }
}
