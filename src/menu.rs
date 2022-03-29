use crate::platform_impl::{
  key, menu::MenuType, menu_item::MenuItem, native_menu_item::NativeMenuItem,
};
use cocoa::{
  appkit::{NSApp, NSApplication, NSMenu, NSMenuItem},
  base::{id, nil, NO},
  foundation::{NSAutoreleasePool, NSString},
};
use objc::{msg_send, runtime::Sel, sel, sel_impl};

pub fn set_menu(menu: &Menu) {
  unsafe {
    let app = NSApp();
    app.setMainMenu_(menu.ns_menu);
  }
}

// Menu Bar
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
    let menu_item = MenuItem::new(title, selector, key_equivalent, MenuType::MenuBar);
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
    let native_menu_item = MenuItem::new_native(item, title, key_equivalent, MenuType::MenuBar);
    unsafe {
      self.ns_menu.addItem_(native_menu_item.ns_menu_item);
    }
  }
  pub fn add_submenu(&self, submenu: &Menu, title: &str) {
    submenu.set_title(title);

    let menu_item = MenuItem::new(title, None, None, MenuType::MenuBar);

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

impl Default for Menu {
  fn default() -> Self {
    Self::new()
  }
}

// Context Menu
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
