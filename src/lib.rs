//! Yet Another Entity Component System
//! #![deny(missing_docs)]
extern crate anymap;

use anymap::AnyMap;

use std::ops::Index;

use std::any::Any;
use std::fmt::{Debug, Formatter, Result};

/// An `Entity` is simply an identifier for a bag of components. In general, `System`s operate on
/// a subset of all entities that posess components the `System` is interested in.
#[derive(Debug)]
pub struct Entity {
    /// A user-defined label for this entity. This could be thrown out if in future we run into
    /// memory issues, but for now its convenient as it allows us to more easily identify an entity.
    pub label: String,
    /// Bag of components
    pub components: AnyMap,
}

impl Entity {
    /// Creates a new `Entity` with an empty bag of components
    pub fn new(label: &'static str) -> Entity {
        Entity {
            label: String::from(label),
            components: AnyMap::new(),
        }
    }
}

pub struct Entities(Vec<Entity>);

impl Entities {
    pub fn new() -> Entities {
        Entities(vec![])
    }

    pub fn push(&mut self, entity: Entity) {
        self.0.push(entity);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn with_component<T: 'static>(&self) -> Vec<&T> {
        self.0
            .iter()
            .filter_map(|e| e.components.get::<T>())
            .collect()
    }

    pub fn with_component_mut<T: 'static>(&mut self) -> Vec<&mut T> {
        self.0
            .iter_mut()
            .filter_map(|e| e.components.get_mut::<T>())
            .collect()
    }
}

impl Index<usize> for Entities {
    type Output = Entity;

    fn index(&self, index: usize) -> &Entity {
        &self.0[index]
    }
}

pub struct EntityBuilder(Entity);

impl EntityBuilder {
    pub fn create(label: &'static str) -> EntityBuilder {
        EntityBuilder(Entity::new(label))
    }

    pub fn add<T: Any>(mut self, component: T) -> EntityBuilder {
        self.0.components.insert(component);
        self
    }

    pub fn build(self) -> Entity {
        self.0
    }
}

pub type Globals = AnyMap;

pub trait System {
    fn process(&self, entities: &mut Entities, globals: &mut Globals);
}

impl Debug for System {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "System")
    }
}

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
    use anymap::AnyMap;

    #[derive(Debug, PartialEq)]
    struct TestComponent(pub u8);

    struct AnotherComponent;

    #[derive(Debug, PartialEq)]
    struct TestSystem;

    impl System for TestSystem {
        fn process(&self, _: &mut Entities, _: &mut AnyMap) {}
    }

    #[test]
    fn entity_can_be_built() {
        let entity = EntityBuilder::create("test").build();
        assert_eq!(entity.label, "test");
    }

    #[test]
    fn entity_can_contain_components() {
        let entity = EntityBuilder::create("test")
            .add(TestComponent(1))
            .build();
        assert_eq!(entity.label, "test");
        assert_eq!(entity.components.get::<TestComponent>(),
                   Some(&TestComponent(1)));
    }

    #[test]
    fn entity_has_components_works() {
        let entity = EntityBuilder::create("test")
            .add(TestComponent(1))
            .build();
        assert!(entity.components.contains::<TestComponent>());
        assert!(!entity.components.contains::<AnotherComponent>());

        let entity = EntityBuilder::create("test")
            .add(AnotherComponent)
            .build();
        assert!(!entity.components.contains::<TestComponent>());
        assert!(entity.components.contains::<AnotherComponent>());
    }

    #[test]
    fn entities_with_component_works() {
        let mut entities = Entities::new();
        entities.push(EntityBuilder::create("test")
            .add(TestComponent(1))
            .build());
        entities.push(EntityBuilder::create("test")
            .add(AnotherComponent)
            .build());

        let test_component = entities.with_component::<TestComponent>().pop();
        assert_eq!(test_component, Some(&TestComponent(1)));
    }

    #[test]
    fn entities_with_component_mut_works() {
        let mut entities = Entities::new();
        entities.push(EntityBuilder::create("test")
            .add(TestComponent(1))
            .build());
        entities.push(EntityBuilder::create("test")
            .add(AnotherComponent)
            .build());

        let test_component = entities.with_component_mut::<TestComponent>().pop();
        assert_eq!(test_component, Some(&mut TestComponent(1)));

        let mut unwrapped = test_component.unwrap();
        unwrapped.0 = 2;
        assert_eq!(unwrapped, &mut TestComponent(2));
    }

    #[test]
    fn world_can_be_created() {
        let world = World::new();
        assert!(world.entities.is_empty());
    }

    #[test]
    fn world_can_contain_entities() {
        let mut world = World::new();
        world.add_entity(EntityBuilder::create("test")
            .add(TestComponent(1))
            .build());

        assert!(!world.entities.is_empty());
        let ref entity = world.entities[0];
        assert_eq!(entity.label, "test");
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
