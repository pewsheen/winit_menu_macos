use crate::key;
use cocoa::{
  appkit::{NSApp, NSApplication, NSMenu, NSMenuItem},
  base::{id, nil, NO},
  foundation::{NSAutoreleasePool, NSString},
};
use objc::{msg_send, sel, sel_impl};

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
  pub fn add_item(&self, menu_item: &MenuItem) {
    unsafe {
      self.ns_menu.addItem_(menu_item.ns_menu_item);
    }
  }
  pub fn add_submenu(&self, submenu: &Menu, title: &str) {
    submenu.set_title(title);

    let menu_item = MenuItem::new(title, None);
    menu_item.add_submenu(&submenu);

    self.add_item(&menu_item);
  }
  pub fn set_title(&self, title: &str) {
    unsafe {
      let menu_title = NSString::alloc(nil).init_str(title);
      let () = msg_send![self.ns_menu, setTitle: menu_title];
    }
  }
}

#[derive(Debug, Clone)]
pub struct MenuItem {
  pub ns_menu_item: id,
}

impl MenuItem {
  pub fn new(title: &str, key_equivalent: Option<key::KeyEquivalent>) -> Self {
    unsafe {
      let _title = NSString::alloc(nil).init_str(title);
      let sel = sel!(fireMenubarAction:);
      let (key, masks) = match key_equivalent {
        Some(ke) => (NSString::alloc(nil).init_str(ke.key), ke.masks),
        None => (NSString::alloc(nil).init_str(""), None),
      };

      let ns_menu_item =
        NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(_title, sel, key);

      if let Some(masks) = masks {
        ns_menu_item.setKeyEquivalentModifierMask_(masks)
      }

      Self { ns_menu_item }
    }
  }
  pub fn add_submenu(&self, submenu: &Menu) {
    unsafe {
      self.ns_menu_item.setSubmenu_(submenu.ns_menu);
    }
  }
}
