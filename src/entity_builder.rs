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

    pub fn tag(mut self, key: &'static str, value: String) -> EntityBuilder {
        self.0.tags.insert(key, value);
        self
    }

    pub fn build(self) -> Entity {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_can_be_built() {
        let entity = EntityBuilder::create_str("test").build();
        assert_eq!(entity.label(), "test");
    }

    #[test]
    fn entity_add_adds_a_component() {
        let entity = EntityBuilder::create_str("test").add("String").build();
        assert_eq!(entity.label(), "test");
        assert_eq!(entity.components.get::<&str>(), Some(&"String"));
    }

    #[test]
    fn entity_tag_adds_a_tag() {
        let entity = EntityBuilder::create_str("test").tag("key", String::from("value")).build();
        assert_eq!(entity.label(), "test");
        assert_eq!(entity.tags.get("key"), Some(&String::from("value")));
    }
}
