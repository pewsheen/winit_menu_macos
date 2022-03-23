use crate::{
  event::Event,
  event_channel::get_event_channel,
  key,
  menu::{Menu, MenuId},
};
use cocoa::{
  appkit::{NSEventModifierFlags, NSMenuItem},
  base::{id, nil},
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

static BLOCK_PTR: &str = "MenuItemBlockPtr";

#[derive(Debug)]
struct Action(Box<u16>);

#[derive(Debug, Clone)]
pub struct MenuItem {
  pub ns_menu_item: id,
}

impl MenuItem {
  pub fn new(
    title: &str,
    selector: Option<Sel>,
    key_equivalent: Option<key::KeyEquivalent>,
  ) -> Self {
    let alloc = make_menu_alloc();
    let menu_id = MenuId::new(title);
    let id = Box::new(Action(Box::new(menu_id.0)));
    let ptr = Box::into_raw(id);

    unsafe {
      (&mut *alloc).set_ivar(BLOCK_PTR, ptr as usize);
      let _: () = msg_send![&*alloc, setTarget:&*alloc];
      let title = NSString::alloc(nil).init_str(title);
      Self {
        ns_menu_item: make_menu_item_from_alloc(alloc, title, selector, key_equivalent),
      }
    }
  }
  pub fn add_submenu(&self, submenu: &Menu) {
    unsafe {
      self.ns_menu_item.setSubmenu_(submenu.ns_menu);
    }
  }
}

fn make_menu_alloc() -> *mut Object {
  unsafe { msg_send![make_menu_item_class(), alloc] }
}

fn make_menu_item_class() -> *const Class {
  static mut APP_CLASS: *const Class = 0 as *const Class;
  static INIT: Once = Once::new();

  INIT.call_once(|| unsafe {
    let superclass = class!(NSMenuItem);
    let mut decl = ClassDecl::new("MenuItem", superclass).unwrap();
    decl.add_ivar::<usize>(BLOCK_PTR);

    decl.add_method(
      sel!(dealloc),
      dealloc_custom_menuitem as extern "C" fn(&Object, _),
    );

    decl.add_method(
      sel!(fireMenubarAction:),
      fire_menu_bar_click as extern "C" fn(&Object, _, id),
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
      None => sel!(fireMenubarAction:),
    };

    // allocate our item to our class
    let item: id = msg_send![alloc, initWithTitle: title action: selector keyEquivalent: key];

    item.setKeyEquivalentModifierMask_(masks);
    item
  }
}

extern "C" fn fire_menu_bar_click(this: &Object, _: Sel, _item: id) {
  send_event(this);
}

extern "C" fn dealloc_custom_menuitem(this: &Object, _: Sel) {
  unsafe {
    let ptr: usize = *this.get_ivar(BLOCK_PTR);
    let obj = ptr as *mut Action;
    if !obj.is_null() {
      let _handler = Box::from_raw(obj);
    }
    let _: () = msg_send![super(this, class!(NSMenuItem)), dealloc];
  }
}

fn send_event(this: &Object) {
  let channel = get_event_channel();
  let tx = channel.0.clone();

  let menu_id = unsafe {
    let ptr: usize = *this.get_ivar(BLOCK_PTR);
    let obj = ptr as *const Action;
    &*obj
  };

  let window_id = unsafe {
    let app: id = msg_send![class!(NSApplication), sharedApplication];
    let window_id: id = msg_send![app, mainWindow];
    Some(get_window_id(window_id))
  };

  let event = Event::MenuEvent {
    window_id,
    menu_id: MenuId(*menu_id.0),
  };

  tx.send(event).unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub usize);

impl Id {
  pub const unsafe fn dummy() -> Self {
    Id(0)
  }
}

// Convert the `cocoa::base::id` associated with a window to a usize to use as a unique identifier
// for the window.
fn get_window_id(window_cocoa_id: id) -> Id {
  Id(window_cocoa_id as *const Object as _)
}
