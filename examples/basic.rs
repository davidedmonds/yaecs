#[macro_use] extern crate yaecs;

use yaecs::{Component, EntityBuilder};

struct BasicComponent(u8);

struct OtherBasicComponent;

components!(BasicComponent, OtherBasicComponent);

fn main() {
  let entity = EntityBuilder::create("test")
                            .add(BasicComponent(1))
                            .add(OtherBasicComponent {})
                            .build();

  println!("{:?}", entity);
}
