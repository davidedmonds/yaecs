//! Components
/// A component should be a bundle of data defining a particular property of a Game Entity.
use std::collections::HashMap;

pub trait Component {
  /// An identifying value, used as a key in component storage maps.
  fn id(&self) -> &'static str;
}

pub struct ComponentStore {
  components: HashMap<&'static str, Box<Component>>
}

impl ComponentStore {
  pub fn new() -> ComponentStore {
    ComponentStore { components: HashMap::new() }
  }

  pub fn add(&mut self, component: Box<Component>) {
    self.components.insert(component.id(), component);
  }
}
