use crate::{
  key,
  menu_item::{MenuItem, MenuType},
  native_menu_item::NativeMenuItem,
};
use cocoa::{
  appkit::{NSMenu, NSMenuItem},
  base::{id, nil, NO},
  foundation::{NSAutoreleasePool, NSString},
};
use objc::{msg_send, runtime::Sel, sel, sel_impl};

#[derive(Debug, Clone)]
pub struct ContextMenu {
  pub ns_menu: id,
}

impl ContextMenu {
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
    let menu_item = MenuItem::new(title, selector, key_equivalent, MenuType::ContextMenu);
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
    let native_menu_item = MenuItem::new_native(item, title, key_equivalent, MenuType::ContextMenu);
    unsafe {
      self.ns_menu.addItem_(native_menu_item.ns_menu_item);
    }
  }
  pub fn add_submenu(&self, submenu: &ContextMenu, title: &str) {
    submenu.set_title(title);

    let menu_item = MenuItem::new(title, None, None, MenuType::ContextMenu);

    unsafe {
      menu_item.ns_menu_item.setSubmenu_(submenu.ns_menu);
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

impl Default for ContextMenu {
  fn default() -> Self {
    Self::new()
  }
}
