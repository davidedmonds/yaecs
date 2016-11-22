//! `System`s operate on `Entity`s, using and modifying their `Component`s
use entity::Entity;
use global::Globals;
use std::any::TypeId;
use world::World;

/// A `System` operates on a `Vec<&Entity>`, either mutating the `Entity`, or using the `Entity` to
/// mutate `Globals`.
pub trait System {

  /// A `Vec<&TypeId>`, listing the component types this system operates on. At present, this is
  /// used only to filter the Entities, and does not affect the components available on each entity.
  /// This means that if you have an `Entity` with three components (A, B and C), and a `System`
  /// that depends on A, `process` will still be able to access and mutate B and C, though without
  /// any certainty that they are present on any of the returned entities.
  fn operates_on(&self) -> Vec<TypeId>;

  /// Operate on the supplied `Vec<&Entity>` and `&Globals` according to the purpose of
  /// this `System`.
  fn process(&self, world: &World, entities: Vec<&Entity>, globals: &Globals);
}
