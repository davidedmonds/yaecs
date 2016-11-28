//! Yet Another Entity Component System
// #![deny(missing_docs)]
extern crate anymap;

use anymap::AnyMap;

use std::any::Any;
use std::fmt::{Debug, Formatter, Result};

///
pub trait Component {
  fn mask() -> u64 where Self: Sized;
  fn fmt(&self) -> String;
}

impl Debug for Component {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "Component {}", self.fmt())
  }
}

/// Macro that generates a bitflag implementation for all supplied components. This allows us to
/// much more quickly determine whether an `Entity` has the required components for a `System`.
/// Note that we are limited to 64 component types with this method.
#[macro_export]
macro_rules! components {
  ($($t:ident),+) => {
    /// An enumeration of all the possible component types.
    enum Components {
      $($t),+
    }

    $(
      /// Adds the `mask()` method to each component, returning the generated bitflag value.
      impl Component for $t {
        /// Returns a bitflag used to identify each component.
        fn mask() -> u64 {
          1 << (Components::$t as u64)
        }

        fn fmt(&self) -> String {
          String::from("$t")
        }
      }
    )+
  };
}

/// An `Entity` is simply an identifier for a bag of components. In general, `System`s operate on
/// a subset of all entities that posess components the `System` is interested in.
#[derive(Debug)]
pub struct Entity {
  /// A user-defined label for this entity. This could be thrown out if in future we run into
  /// memory issues, but for now its convenient as it allows us to more easily identify an entity.
  pub label: String,
  /// Bitmask, indicating which components are implemented for this type.
  pub component_mask: u64,
  /// Bag of components
  pub components: AnyMap
}

impl Entity {
  /// Creates a new `Entity` with an empty bag of components
  pub fn new(label: &'static str) -> Entity {
    Entity {
      label: String::from(label),
      component_mask: 0,
      components: AnyMap::new(),
    }
  }

  pub fn add<T>(&mut self, component: T) where T: Component + Any {
    self.component_mask = self.component_mask | T::mask();
    self.components.insert(component);
  }
}

pub struct EntityBuilder(Entity);

impl EntityBuilder {
  pub fn create(label: &'static str) -> EntityBuilder {
    EntityBuilder(Entity::new(label))
  }

  pub fn add<T>(mut self, component: T) -> EntityBuilder where T: Component + Any {
    self.0.add(component);
    self
  }

  pub fn build(self) -> Entity {
    self.0
  }
}

pub trait System {
  fn process(&self, entities: &mut Vec<Entity>, globals: &mut AnyMap);
}

impl Debug for System {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "System")
  }
}

pub struct World {
  entities: Vec<Entity>,
  globals: AnyMap,
  systems: Vec<Box<System>>,
}

impl World {
  pub fn new() -> World {
    World {
      entities: vec!(),
      globals: AnyMap::new(),
      systems: vec!(),
    }
  }

  pub fn add_entity(&mut self, entity: Entity) {
    self.entities.push(entity);
  }

  pub fn add_global<T: Any>(&mut self, global: T) {
    self.globals.insert(global);
  }

  pub fn add_system<T: System + 'static>(&mut self, system: T) {
    self.systems.push(Box::new(system));
  }

  pub fn update(&mut self) {
    for system in &self.systems {
      (*system).process(&mut self.entities, &mut self.globals);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use anymap::AnyMap;

  #[derive(Debug, PartialEq)]
  struct TestComponent(u8);

  components!(TestComponent);

  #[derive(Debug, PartialEq)]
  struct TestSystem;

  impl System for TestSystem {
    fn process(&self, _: &mut Vec<Entity>, _: &mut AnyMap) { }
  }

  #[test]
  fn entity_can_be_built() {
    let entity = EntityBuilder::create("test").build();
    assert_eq!(entity.label, "test");
  }

  #[test]
  fn entity_can_contain_components() {
    let entity = EntityBuilder::create("test")
                                .add(TestComponent(1))
                                .build();
    assert_eq!(entity.label, "test");
    assert_eq!(entity.component_mask, TestComponent::mask());
    assert_eq!(entity.components.get(), Some(&TestComponent(1)));
  }

  #[test]
  fn world_can_be_created() {
    let world = World::new();
    assert!(world.entities.is_empty());
  }

  #[test]
  fn world_can_contain_entities() {
    let mut world = World::new();
    world.add_entity(EntityBuilder::create("test")
                                  .add(TestComponent(1))
                                  .build());

    assert!(!world.entities.is_empty());
    let ref entity = world.entities[0];
    assert_eq!(entity.label, "test");
    assert_eq!(entity.component_mask, TestComponent::mask());
    assert_eq!(entity.components.get(), Some(&TestComponent(1)));
  }

  #[test]
  fn world_can_contain_systems() {
    let mut world = World::new();

    assert!(world.systems.is_empty());
    world.add_system(TestSystem);
    assert!(!world.systems.is_empty());
  }
}
