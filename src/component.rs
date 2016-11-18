//! Components
/// A component should be a bundle of data defining a particular property of a Game Entity.
pub trait Component {
  /// An identifying value, used as a key in component storage maps.
  fn id(&self) -> &'static str;
}
