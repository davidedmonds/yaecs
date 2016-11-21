//! Entity
use component::ComponentStore;

/// An `Entity` is simply an identifier for a bag of components. In general, `System`s operate on
/// a `Vec<&Entity>` which is a filtered view based on which components the `System` is
/// interested in.
pub struct Entity {
  /// The bag of components.
  pub components: ComponentStore
}

impl Entity {
  /// Creates a new `Entity` with an empty bag of components
  pub fn new() -> Entity {
    Entity { components: ComponentStore::new() }
  }
}
