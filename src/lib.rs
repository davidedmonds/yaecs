//! Yet Another Entity Component System
#![deny(missing_docs)]
extern crate anymap;

use anymap::AnyMap;
use std::any::TypeId;

/// A `ComponentStore` is the storage mechanism for a component. Only one instance of any single
/// component can be stored in a RawMap, which is consistent with how Entities and Components
/// generally behave in an ECS.
pub type ComponentStore = AnyMap;

/// An `Entity` is simply an identifier for a bag of components. In general, `System`s operate on
/// a `Vec<&Entity>` which is a filtered view based on which components the `System` is
/// interested in.
#[derive(Debug)]
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

/// `Globals` is similar to `ComponentStore`, but where `ComponentStore`s are tied to `Entities`,
/// this is tied to the `World` instead. This means that values stored in here are available to
/// every `System` globally. This is particularly useful for access to shared resources, such as
/// the display window or control inputs.
pub type Globals = AnyMap;

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

/// This holds the `Entity`s and `Globals`, to allow this to be passed in as a whole to
/// `System.process` without upsetting the borrow checker.
#[derive(Debug)]
pub struct WorldData {
  /// All entities are stored in this `Vec`. Ideally `create_entity` should be used to add to this
  /// collection, as it conveniently allows the `ComponentStore` to be set at the same time as well.
  pub entities: Vec<Entity>,
  /// All 'world-level' components are stored in this object (a Typedef'd `AnyMap`), and as such
  /// only one instance of each component type can be stored here. Intended to be added to directly
  /// using the methods on `Globals` from `AnyMap`.
  pub globals: Globals,
}

impl WorldData {
  /// Creates a new WorldData with empty `Entity`s and `Globals`
  pub fn new() -> WorldData {
    WorldData {
      entities: vec!(),
      globals: Globals::new(),
    }
  }

  /// Convenience method to add a new entity to the world, populated using the callback supplied.
  pub fn create_entity<CB>(&mut self, cb: CB) where CB: Fn(&mut ComponentStore) -> () {
    let mut entity = Entity::new();
    cb(&mut entity.components);
    self.entities.push(entity);
  }
}

/// The root of the ECS system, a `World` is the point at which all `Entity`s, `Globals` and
/// `System`s are owned. All accesses downstream from this are on borrows.
pub struct World {
  /// All `System`s that operate on components are stored within this `Vec`. As `System` is a
  /// trait, we have to box all systems that are added here (it might be possible to avoid that?
  /// answers on a pull-request ^_^ ) and so subsequently all method calls for systems as well.
  pub systems: Vec<Box<System>>,
  /// Container for `Entity`s and `Globals`
  pub world_data: WorldData,
}

impl World {
  /// Creates a new world, with all three collections empty.
  pub fn new() -> World {
    World {
      systems: vec!(),
      world_data: WorldData::new(),
    }
  }

  /// Convenience method to add a new entity to the world, populated using the callback supplied.
  pub fn create_entity<CB>(&mut self, cb: CB) where CB: Fn(&mut ComponentStore) -> () {
    self.world_data.create_entity(cb);
  }

  /// Register a system in the world.
  pub fn register(&mut self, system: Box<System>) {
    self.systems.push(system);
  }

  /// Updates the world, by calling all systems in turn with a the entire world. Currently
  /// completely un-optimized.
  pub fn update(&mut self) {
    for system in self.systems.iter() {
      system.process(&mut self.world_data);
    }
  }
}
