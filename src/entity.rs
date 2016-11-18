//! Entity
use component::ComponentStore;

pub struct Entity {
  pub components: ComponentStore
}

impl Entity {
  pub fn new() -> Entity {
    Entity { components: ComponentStore::new() }
  }
}
