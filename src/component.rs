//! Components
/// A component should be a bundle of data defining a particular property of a Game Entity.
use anymap::raw::RawMap;

/// A `ComponentStore` is the storage mechanism for a component. Only one instance of any single
/// component can be stored in an AnyMap, which is consistent with how Entities and Components
/// generally behave in an ECS.
pub type ComponentStore = RawMap;
