use crate::{
  event::Event,
  event_channel::get_event_channel,
  platform_impl::{
    key,
    menu::{get_window_id, MenuId, MenuType},
    native_menu_item_type::{make_native_menu_item, NativeMenuItemType},
  },
};
use cocoa::{
  appkit::{NSButton, NSEventModifierFlags, NSMenuItem},
  base::{id, nil, NO, YES},
  foundation::NSString,
};
use objc::{
  class,
  declare::ClassDecl,
  msg_send,
  runtime::{Class, Object, Sel},
  sel, sel_impl,
};
use std::sync::Once;

static MENU_IDENTITY: &str = "MenuItemIdentity";

#[derive(Debug, Clone)]
pub struct MenuItem {
  pub ns_menu_item: id,
  menu_item_id: Option<MenuId>,
}

impl MenuItem {
  pub fn id(&self) -> MenuId {
    self.menu_item_id.unwrap_or(MenuId::EMPTY)
  }
  pub fn new(
    title: &str,
    selector: Option<Sel>,
    key_equivalent: Option<key::KeyEquivalent>,
    enabled: bool,
    selected: bool,
    menu_type: MenuType,
  ) -> Self {
    let (menu_item_id, menu_item) = make_menu_item(title, selector, key_equivalent, menu_type);
    unsafe {
      (&mut *menu_item).set_ivar(MENU_IDENTITY, menu_item_id.unwrap().0);
      let _: () = msg_send![&*menu_item, setTarget:&*menu_item];

      if selected {
        let () = msg_send![menu_item, setState: 1_isize];
      }
      if !enabled {
        let () = msg_send![menu_item, setEnabled: NO];
      }
    }
    Self {
      ns_menu_item: menu_item,
      menu_item_id,
    }
  }
  pub fn new_native(
    item: NativeMenuItemType,
    title: Option<&str>,
    key_equivalent: Option<key::KeyEquivalent>,
    menu_type: MenuType,
  ) -> Self {
    Self {
      ns_menu_item: make_native_menu_item(item, title, key_equivalent, menu_type),
      menu_item_id: None,
    }
  }
  pub fn set_enabled(&mut self, is_enabled: bool) -> &Self {
    unsafe {
      let status = match is_enabled {
        true => YES,
        false => NO,
      };
      let () = msg_send![self.ns_menu_item, setEnabled: status];
    }
    self
  }
  pub fn set_title(&mut self, title: &str) -> &Self {
    unsafe {
      let menu_title = NSString::alloc(nil).init_str(title);
      self.ns_menu_item.setTitle_(menu_title);
    }
    self
  }
  pub fn set_selected(&mut self, is_selected: bool) -> &Self {
    unsafe {
      let state = match is_selected {
        true => 1_isize,
        false => 0_isize,
      };
      let () = msg_send![self.ns_menu_item, setState: state];
    }
    self
  }
}

pub fn make_menu_item(
  title: &str,
  selector: Option<Sel>,
  key_equivalent: Option<key::KeyEquivalent>,
  menu_type: MenuType,
) -> (Option<MenuId>, *mut Object) {
  let alloc = make_menu_item_alloc();
  let menu_id = MenuId::new(title);

  unsafe {
    let title = NSString::alloc(nil).init_str(title);
    let menu_item = make_menu_item_from_alloc(alloc, title, selector, key_equivalent, menu_type);

    (Some(menu_id), menu_item)
  }
}

fn make_menu_item_alloc() -> *mut Object {
  unsafe { msg_send![make_menu_item_class(), alloc] }
}

fn make_menu_item_class() -> *const Class {
  static mut APP_CLASS: *const Class = 0 as *const Class;
  static INIT: Once = Once::new();

  INIT.call_once(|| unsafe {
    let superclass = class!(NSMenuItem);
    let mut decl = ClassDecl::new("MenuItem", superclass).unwrap();
    decl.add_ivar::<u16>(MENU_IDENTITY);

    decl.add_method(
      sel!(dealloc),
      dealloc_custom_menuitem as extern "C" fn(&Object, _),
    );

    decl.add_method(
      sel!(fireMenubarAction:),
      fire_menu_bar_click as extern "C" fn(&Object, _, id),
    );

    decl.add_method(
      sel!(fireStatusbarAction:),
      fire_status_bar_click as extern "C" fn(&Object, _, id),
    );

    APP_CLASS = decl.register();
  });

  unsafe { APP_CLASS }
}

fn make_menu_item_from_alloc(
  alloc: *mut Object,
  title: *mut Object,
  selector: Option<Sel>,
  key_equivalent: Option<key::KeyEquivalent>,
  menu_type: MenuType,
) -> *mut Object {
  unsafe {
    let (key, masks) = match key_equivalent {
      Some(ke) => (
        NSString::alloc(nil).init_str(ke.key),
        ke.masks.unwrap_or_else(NSEventModifierFlags::empty),
      ),
      None => (
        NSString::alloc(nil).init_str(""),
        NSEventModifierFlags::empty(),
      ),
    };

    let selector = match selector {
      Some(selector) => selector,
      None => match menu_type {
        MenuType::MenuBar => sel!(fireMenubarAction:),
        MenuType::ContextMenu => sel!(fireStatusbarAction:),
      },
    };

    // allocate our item to our class
    let item: id = msg_send![alloc, initWithTitle: title action: selector keyEquivalent: key];

    item.setKeyEquivalentModifierMask_(masks);
    item
  }
}

extern "C" fn fire_menu_bar_click(this: &Object, _: Sel, _item: id) {
  send_event(this, MenuType::MenuBar);
}

extern "C" fn fire_status_bar_click(this: &Object, _: Sel, _item: id) {
  send_event(this, MenuType::ContextMenu);
}

extern "C" fn dealloc_custom_menuitem(this: &Object, _: Sel) {
  unsafe {
    let _: () = msg_send![super(this, class!(NSMenuItem)), dealloc];
  }
}

fn send_event(this: &Object, menu_type: MenuType) {
  let channel = get_event_channel();
  let tx = channel.0.clone();

  let menu_id = unsafe {
    let id: u16 = *this.get_ivar(MENU_IDENTITY);
    id
  };

  let window_id = match menu_type {
    MenuType::MenuBar => unsafe {
      let app: id = msg_send![class!(NSApplication), sharedApplication];
      let window_id: id = msg_send![app, mainWindow];
      Some(get_window_id(window_id))
    },
    MenuType::ContextMenu => None,
  };

  let event = Event::MenuEvent {
    window_id,
    menu_id: MenuId(menu_id),
    menu_type: menu_type,
  };

  tx.send(event).unwrap();
}
