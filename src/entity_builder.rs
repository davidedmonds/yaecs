use ::Entity;
use std::any::Any;

pub struct EntityBuilder(Entity);

impl EntityBuilder {
    pub fn create(label: String) -> EntityBuilder {
        EntityBuilder(Entity::new(label))
    }

    pub fn create_str(label: &str) -> EntityBuilder {
        EntityBuilder(Entity::new(String::from(label)))
    }

    pub fn add<T: Any>(mut self, component: T) -> EntityBuilder {
        self.0.components.insert(component);
        self
    }

    pub fn build(self) -> Entity {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anymap::AnyMap;

    #[test]
    fn entity_can_be_built() {
        let entity = EntityBuilder::create_str("test").build();
        assert_eq!(entity.label, "test");
    }
}
