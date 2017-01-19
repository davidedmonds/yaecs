#[macro_use]
extern crate yaecs;

use yaecs::EntityBuilder;

struct BasicComponent(u8);

struct OtherBasicComponent;

fn main() {
    let entity = EntityBuilder::create_str("test")
        .add(BasicComponent(1))
        .add(OtherBasicComponent {})
        .build();

    println!("{:?}", entity);
}
