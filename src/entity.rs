use anymap::AnyMap;
use std::collections::HashMap;

/// An `Entity` is simply an identifier for a bag of components. In general, `System`s operate on
/// a subset of all entities that posess components the `System` is interested in.
#[derive(Debug)]
pub struct Entity {
    /// Bag of components
    pub components: AnyMap,
    /// Tags (metadata about this entity)
    pub tags: HashMap<&'static str, String>,
}

impl Entity {
    /// Creates a new `Entity` with an empty bag of components
    pub fn new(label: String) -> Entity {
        let mut tags = HashMap::new();
        tags.insert("label", label);
        Entity {
            components: AnyMap::new(),
            tags: tags,
        }
    }

    pub fn label(&self) -> &str {
        self.tags.get("label").unwrap()
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.label() == other.label()
    }
}

#[cfg(test)]
mod tests {
    use ::EntityBuilder;

    #[derive(Debug, PartialEq)]
    struct TestComponent(pub u8);

    #[derive(Debug, PartialEq)]
    struct AnotherComponent;

    #[test]
    fn entity_can_contain_components() {
        let entity = EntityBuilder::create_str("test")
            .add(TestComponent(1))
            .build();
        assert_eq!(entity.tags.get("label"), Some(&String::from("test")));
        assert_eq!(entity.components.get::<TestComponent>(),
                   Some(&TestComponent(1)));
    }

    #[test]
    fn entity_has_components_works() {
        let entity = EntityBuilder::create_str("test")
            .add(TestComponent(1))
            .build();
        assert!(entity.components.contains::<TestComponent>());
        assert!(!entity.components.contains::<AnotherComponent>());

        let entity = EntityBuilder::create_str("test")
            .add(AnotherComponent)
            .build();
        assert!(!entity.components.contains::<TestComponent>());
        assert!(entity.components.contains::<AnotherComponent>());
    }
}
