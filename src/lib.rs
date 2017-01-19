//! Yet Another Entity Component System
//! #![deny(missing_docs)]
extern crate anymap;

use anymap::AnyMap;

mod entities;
mod entity_builder;
mod entity;
mod system;
mod world;

pub use entities::Entities;
pub use entity_builder::EntityBuilder;
pub use entity::Entity;
pub use system::System;
pub use world::World;

pub type Globals = AnyMap;
