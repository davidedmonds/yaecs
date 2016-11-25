#[macro_use] extern crate yaecs;

use yaecs::Component;

struct BasicComponent(u8);

struct OtherBasicComponent;

components!(BasicComponent, OtherBasicComponent);

fn main() {
  unimplemented!();
}
