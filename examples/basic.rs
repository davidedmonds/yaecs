extern crate yaecs;

use yaecs::system::System;
use yaecs::world::{World, WorldData};

fn main() {
  let mut world = World::new();
  world.register(Box::new(BasicSystem::new()));
  world.create_entity(| components | {
    components.insert(BasicComponent(128));
    components.insert(OtherBasicComponent {});
  });
  world.update();
}

struct BasicComponent(u8);

struct OtherBasicComponent;

struct BasicSystem;

impl BasicSystem {
  fn new() -> BasicSystem {
    BasicSystem { }
  }
}

impl System for BasicSystem {
  fn process(&self, world_data: &mut WorldData) {
    println!("Processing world_data {:?}", world_data);
  }
}
