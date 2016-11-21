//! Reexport AnyMap as a library type to make working with the ECS easier.
use anymap::AnyMap;

/// `Globals` is similar to `ComponentStore`, but where `ComponentStore`s are tied to `Entities`,
/// this is tied to the `World` instead. This means that values stored in here are available to
/// every `System` globally. This is particularly useful for access to shared resources, such as
/// the display window or control inputs.
pub type Globals = AnyMap;
