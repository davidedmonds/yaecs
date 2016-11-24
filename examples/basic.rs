extern crate yaecs;

use std::any::TypeId;

use yaecs::entity::Entity;
use yaecs::global::Globals;
use yaecs::system::{EntitySystem, System};
use yaecs::world::{World, WorldData};

fn main() {
  let mut world = World::new();
  world.register(Box::new(BasicSystem::new()));
  world.register(Box::new(BasicEntitySystem {}));
  world.create_entity(| components | {
    components.insert(BasicComponent(128));
    components.insert(OtherBasicComponent {});
  });
  world.create_entity(| _ | {});
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

struct BasicEntitySystem;

impl EntitySystem for BasicEntitySystem {
  fn operates_on(&self) -> Vec<TypeId> {
    vec!(TypeId::of::<BasicComponent>())
  }

  fn process_entities(&self, entities: Vec<&Entity>, globals: &Globals) {
    println!("Processing entities {:?} with globals {:?}", entities, globals);
  }
}
