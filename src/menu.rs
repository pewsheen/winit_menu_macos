use crate::{key, menu_item::MenuItem, native_menu_item::NativeMenuItem};
use cocoa::{
  appkit::{NSApp, NSApplication, NSMenu},
  base::{id, nil, NO},
  foundation::{NSAutoreleasePool, NSString},
};
use objc::{msg_send, runtime::Sel, sel, sel_impl};
use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

pub fn set_menu(menu: &Menu) {
  unsafe {
    let app = NSApp();
    app.setMainMenu_(menu.ns_menu);
  }
}

#[derive(Debug, Clone)]
pub struct Menu {
  pub ns_menu: id,
}

impl Menu {
  pub fn new() -> Self {
    unsafe {
      let ns_menu = NSMenu::alloc(nil).autorelease();
      let () = msg_send![ns_menu, setAutoenablesItems: NO];
      Self { ns_menu }
    }
  }
  pub fn add_item(
    &self,
    title: &str,
    selector: Option<Sel>,
    key_equivalent: Option<key::KeyEquivalent>,
  ) {
    let menu_item = MenuItem::new(title, selector, key_equivalent);
    unsafe {
      self.ns_menu.addItem_(menu_item.ns_menu_item);
    }
  }
  pub fn add_native_item(
    &self,
    item: NativeMenuItem,
    title: Option<&str>,
    key_equivalent: Option<key::KeyEquivalent>,
  ) {
    let native_menu_item = MenuItem::new_native(item, title, key_equivalent);
    unsafe {
      self.ns_menu.addItem_(native_menu_item.ns_menu_item);
    }
  }
  pub fn add_submenu(&self, submenu: &Menu, title: &str) {
    submenu.set_title(title);

    let menu_item = MenuItem::new(title, None, None);
    menu_item.add_submenu(&submenu);

    unsafe {
      self.ns_menu.addItem_(menu_item.ns_menu_item);
    }
  }
  pub fn set_title(&self, title: &str) {
    unsafe {
      let menu_title = NSString::alloc(nil).init_str(title);
      let () = msg_send![self.ns_menu, setTitle: menu_title];
    }
  }
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
