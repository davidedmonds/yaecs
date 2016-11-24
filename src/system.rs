//! `System`s operate directly on a `World`.
use entity::Entity;
use global::Globals;
use world::WorldData;

use std::any::TypeId;

/// A `System` operates directly on a `World`.
pub trait System {
  /// Operate on the `World` according to the purpose of this `System`.
  fn process(&self, world_data: &mut WorldData);
}

/// A `System` specialised in working with `Entity`s filtered by component.
pub trait EntitySystem {
  /// `TypeId`s of the components this `System` operate on.
  fn operates_on(&self) -> Vec<TypeId>;

  /// Operate on the `Entity`s using the `Globals`.
  fn process_entities(&self, entities: Vec<&Entity>, globals: &Globals);
}

impl <T: EntitySystem> System for T {
  fn process(&self, world_data: &mut WorldData) {
    let filtered_entities = world_data.entities.iter()
          .filter(| e | self.operates_on().into_iter().all(| id | e.components.contains_key(&id)))
          .collect();
    self.process_entities(filtered_entities, &world_data.globals);
  }
}
