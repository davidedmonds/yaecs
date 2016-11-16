//! `System`s operate on `Entity`s, using and modifying their `Component`s
use component::Component;
use stash::Stash;
use std::collections::HashSet;

pub trait System {
  fn process(&mut self, entities: &Stash<HashSet<Component>>);
}
