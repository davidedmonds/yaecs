//! `System`s operate on `Entity`s, using and modifying their `Component`s
pub trait System {
  fn process(&mut self);
}
