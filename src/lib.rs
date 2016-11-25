//! Yet Another Entity Component System
// #![deny(missing_docs)]

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
          String::from("Component")
        }
      }
    )+
  };
}

/// An `Entity` is simply an identifier for a bag of components. In general, `System`s operate on
/// a subset of all entities that posess components the `System` is interested in.
#[derive(Debug)]
pub struct Entity {
  /// The unique identifier for this entity. TODO is this needed?
  pub id: u64,
  /// A user-defined label for this entity. This could be thrown out if in future we run into
  /// memory issues, but for now its convenient as it allows us to more easily identify an entity.
  pub label: String,
  /// Bitmask, indicating which components are implemented for this type.
  pub component_mask: u64,
  /// Bag of components
  pub components: Vec<Box<Component>>
}

pub struct EntityBuilder(Entity);

impl EntityBuilder {
  pub fn create(label: &'static str) -> EntityBuilder {
    EntityBuilder(Entity::new(label))
  }

  pub fn add<T>(mut self, component: T) -> EntityBuilder where T: Component + 'static {
    self.0.add(component);
    self
  }

  pub fn build(self) -> Entity {
    self.0
  }
}

impl Entity {
  /// Creates a new `Entity` with an empty bag of components
  pub fn new(label: &'static str) -> Entity {
    Entity {
      id: 0,
      label: String::from(label),
      component_mask: 0,
      components: vec!(),
    }
  }

  pub fn add<T>(&mut self, component: T) where T: Component + 'static {
    self.component_mask = self.component_mask | T::mask();
    self.components.push(Box::new(component));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub struct TestComponent(u8);

  components!(TestComponent);

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
  }
}
