//! `System`s operate directly on a `World`.
use world::WorldData;

/// A `System` operates directly on a `World`.
pub trait System {
  /// Operate on the `World` according to the purpose of this `System`.
  fn process(&self, world_data: &mut WorldData);
}
