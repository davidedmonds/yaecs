//! Entity
use component::Component;
use std::collections::HashMap;

pub struct Entity {
  pub components: HashMap<&'static str, Box<Component>>
}
