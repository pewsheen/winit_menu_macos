use cocoa::base::id;
use objc::runtime::Object;
use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

#[derive(Debug, Clone, PartialEq)]
pub enum MenuType {
  MenuBar,
  ContextMenu,
}

/// Identifier of a custom menu item.
///
/// Whenever you receive an event arising from a particular menu, this event contains a `MenuId` which
/// identifies its origin.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MenuId(pub u16);

impl From<MenuId> for u16 {
  fn from(s: MenuId) -> u16 {
    s.0
  }
}

impl MenuId {
  /// Return an empty `MenuId`.
  pub const EMPTY: MenuId = MenuId(0);

  /// Create new `MenuId` from a String.
  pub fn new(unique_string: &str) -> MenuId {
    MenuId(hash_string_to_u16(unique_string))
  }

  /// Whenever this menu is empty.
  pub fn is_empty(self) -> bool {
    Self::EMPTY == self
  }
}

fn hash_string_to_u16(title: &str) -> u16 {
  let mut s = DefaultHasher::new();
  title.to_uppercase().hash(&mut s);
  s.finish() as u16
}

// Store cocoa::base::id
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub usize);

impl Id {
  pub const unsafe fn dummy() -> Self {
    Id(0)
  }
}

// Convert the `cocoa::base::id` associated with a window to a usize to use as a unique identifier
// for the window.
pub fn get_window_id(window_cocoa_id: id) -> Id {
  Id(window_cocoa_id as *const Object as _)
}
