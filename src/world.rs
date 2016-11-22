//! A world is the containing unit that allows the ECS to function.

use component::ComponentStore;
use entity::Entity;
use global::Globals;
use system::System;

/// The root of the ECS system, a `World` is the point at which all `Entity`s, `Globals` and
/// `System`s are owned. All accesses downstream from this are on borrows.
pub struct World {
  /// All entities are stored in this `Vec`. Ideally `create_entity` should be used to add to this
  /// collection, as it conveniently allows the `ComponentStore` to be set at the same time as well.
  pub entities: Vec<Entity>,
  /// All 'world-level' components are stored in this object (a Typedef'd `AnyMap`), and as such
  /// only one instance of each component type can be stored here. Intended to be added to directly
  /// using the methods on `Globals` from `AnyMap`.
  pub globals: Globals,
  /// All `System`s that operate on components are stored within this `Vec`. As `System` is a
  /// trait, we have to box all systems that are added here (it might be possible to avoid that?
  /// answers on a pull-request ^_^ ) and so subsequently all method calls for systems as well.
  pub systems: Vec<Box<System>>
}

impl World {
  /// Creates a new world, with all three collections empty.
  pub fn new() -> World {
    World {
      entities: vec!(),
      globals: Globals::new(),
      systems: vec!()
    }
  }

  /// Convenience method to add a new entity to the world, populated using the callback supplied.
  pub fn create_entity<CB>(&mut self, cb: CB) where CB: Fn(&mut ComponentStore) -> () {
    let mut entity = Entity::new();
    cb(&mut entity.components);
    self.entities.push(entity);
  }

  /// Register a system in the world.
  pub fn register(&mut self, system: Box<System>) {
    self.systems.push(system);
  }

  /// Updates the world, by calling all systems in turn with a list of the entities containing
  /// components that they are interested in. Currently completely un-optimized.
  pub fn update(&mut self) {
    let ref globals = self.globals;
    for system in &self.systems {
      let ref entities = &self.entities;
      let filtered_entities = entities.into_iter()
        .filter(| e | system.operates_on().into_iter().all(| id | e.components.contains_key(&id)))
        .collect();
      system.process(self, filtered_entities, globals);
    }
  }
}
