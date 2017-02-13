use ::{Entity, Entities, Globals, System};
use std::any::Any;

pub struct World {
    entities: Entities,
    globals: Globals,
    systems: Vec<Box<System>>,
}

impl World {
    pub fn new() -> World {
        World {
            entities: Entities::new(),
            globals: Globals::new(),
            systems: vec![],
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn add_global<T: Any>(&mut self, global: T) {
        self.globals.insert(global);
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn get_global<T: Any>(&self) -> Option<&T> {
        self.globals.get::<T>()
    }

    pub fn update(&mut self) {
        for system in &self.systems {
            (*system).process(&mut self.entities, &mut self.globals);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::{Entities, EntityBuilder, System};
    use anymap::AnyMap;

    #[derive(Debug, PartialEq)]
    struct TestComponent(pub u8);

    #[derive(Debug, PartialEq)]
    struct AnotherComponent;

    #[derive(Debug, PartialEq)]
    struct TestSystem;

    impl System for TestSystem {
        fn process(&self, _: &mut Entities, _: &mut AnyMap) {}
    }

    #[test]
    fn world_can_be_created() {
        let world = World::new();
        assert!(world.entities.is_empty());
    }

    #[test]
    fn world_can_contain_entities() {
        let mut world = World::new();
        world.add_entity(EntityBuilder::create_str("test")
            .add(TestComponent(1))
            .build());

        assert!(!world.entities.is_empty());
        let ref entity = world.entities[0];
        assert_eq!(entity.label(), "test");
        assert_eq!(entity.components.get(), Some(&TestComponent(1)));
    }

    #[test]
    fn world_can_contain_systems() {
        let mut world = World::new();

        assert!(world.systems.is_empty());
        world.add_system(TestSystem);
        assert!(!world.systems.is_empty());
    }
}
