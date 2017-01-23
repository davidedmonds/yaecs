use ::Entity;
use std::ops::Index;

pub struct Entities(Vec<Entity>);

impl Entities {
    pub fn new() -> Entities {
        Entities(vec![])
    }

    pub fn push(&mut self, entity: Entity) {
        self.0.push(entity);
    }

    pub fn remove(&mut self, idx: usize) {
        self.0.swap_remove(idx);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn vec(&self) -> Vec<&Entity> {
        self.0.iter().collect()
    }

    pub fn with_label(&self, label: &str) -> Vec<&Entity> {
        self.0
            .iter()
            .filter(|e| e.label == label)
            .collect()
    }

    pub fn with_label_mut(&mut self, label: &str) -> Vec<&mut Entity> {
        self.0
            .iter_mut()
            .filter(|e| e.label == label)
            .collect()
    }

    pub fn with_component<T: 'static>(&self) -> Vec<&Entity> {
        self.0
            .iter()
            .filter(|e| e.components.contains::<T>())
            .collect()
    }

    pub fn with_component_mut<T: 'static>(&mut self) -> Vec<&mut Entity> {
        self.0
            .iter_mut()
            .filter(|e| e.components.contains::<T>())
            .collect()
    }

    pub fn filter_map_to_component<T: 'static>(&self) -> Vec<&T> {
        self.0
            .iter()
            .filter_map(|e| e.components.get::<T>())
            .collect()
    }

    pub fn filter_map_to_component_mut<T: 'static>(&mut self) -> Vec<&mut T> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ::EntityBuilder;

    #[derive(Debug, PartialEq)]
    struct TestComponent(pub u8);

    #[derive(Debug, PartialEq)]
    struct AnotherComponent;

    #[test]
    fn iter_works() {
        let mut entities = Entities::new();
        entities.push(EntityBuilder::create_str("test1")
            .add(TestComponent(1))
            .build());
        entities.push(EntityBuilder::create_str("test2")
            .add(AnotherComponent)
            .build());

        let res = entities.vec();
        assert_eq!(2, res.len());
    }

    #[test]
    fn with_label_works() {
        let mut entities = Entities::new();
        let ent1 = EntityBuilder::create_str("test1")
            .add(TestComponent(1))
            .build();
        let ent2 = EntityBuilder::create_str("test2")
            .add(AnotherComponent)
            .build();

        entities.push(ent1);
        entities.push(ent2);

        let mut results = entities.with_label("test1");
        assert_eq!(1, results.len());
        assert_eq!("test1", results.pop().unwrap().label);
        assert!(entities.with_label("aaaaaa").is_empty());
    }

    #[test]
    fn with_label_mut_works() {
        let mut entities = Entities::new();
        let ent1 = EntityBuilder::create_str("test1")
            .add(TestComponent(1))
            .build();
        let ent2 = EntityBuilder::create_str("test2")
            .add(AnotherComponent)
            .build();

        entities.push(ent1);
        entities.push(ent2);

        {
            let mut results = entities.with_label_mut("test1");
            assert_eq!(1, results.len());

            let mut unwrapped = results.pop().unwrap();
            assert_eq!("test1", unwrapped.label);
            let mut unwrapped_component = unwrapped.components.get_mut::<TestComponent>().unwrap();
            unwrapped_component.0 = 2;
            assert_eq!(unwrapped_component, &mut TestComponent(2));
        }
        assert!(entities.with_label("aaaaaa").is_empty());
    }

    #[test]
    fn with_component_works() {
        let mut entities = Entities::new();
        entities.push(EntityBuilder::create_str("test")
            .add(TestComponent(1))
            .build());
        entities.push(EntityBuilder::create_str("test")
            .add(AnotherComponent)
            .build());

        let test_component = entities.with_component::<TestComponent>().pop().unwrap();
        assert!(test_component.components.contains::<TestComponent>());
    }

    #[test]
    fn with_component_mut_works() {
        let mut entities = Entities::new();
        entities.push(EntityBuilder::create_str("test")
            .add(TestComponent(1))
            .build());
        entities.push(EntityBuilder::create_str("test")
            .add(AnotherComponent)
            .build());

        let test_component = entities.with_component_mut::<TestComponent>().pop().unwrap();
        assert!(test_component.components.contains::<TestComponent>());

        let mut unwrapped = test_component.components.get_mut::<TestComponent>().unwrap();
        unwrapped.0 = 2;
        assert_eq!(unwrapped, &mut TestComponent(2));
    }


    #[test]
    fn filter_map_to_component_works() {
        let mut entities = Entities::new();
        entities.push(EntityBuilder::create_str("test")
            .add(TestComponent(1))
            .build());
        entities.push(EntityBuilder::create_str("test")
            .add(AnotherComponent)
            .build());

        let test_component = entities.filter_map_to_component::<TestComponent>().pop();
        assert_eq!(test_component, Some(&TestComponent(1)));
    }

    #[test]
    fn filter_map_to_component_mut_works() {
        let mut entities = Entities::new();
        entities.push(EntityBuilder::create_str("test")
            .add(TestComponent(1))
            .build());
        entities.push(EntityBuilder::create_str("test")
            .add(AnotherComponent)
            .build());

        let test_component = entities.filter_map_to_component_mut::<TestComponent>().pop();
        assert_eq!(test_component, Some(&mut TestComponent(1)));

        let mut unwrapped = test_component.unwrap();
        unwrapped.0 = 2;
        assert_eq!(unwrapped, &mut TestComponent(2));
    }
}
