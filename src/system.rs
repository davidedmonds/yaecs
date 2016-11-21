//! `System`s operate on `Entity`s, using and modifying their `Component`s
use entity::Entity;
use global::Globals;
use std::any::TypeId;

pub trait System {
  fn operates_on(&self) -> Vec<&TypeId>;

  fn process(&mut self, entities: Vec<&Entity>, globals: &Globals);
}
