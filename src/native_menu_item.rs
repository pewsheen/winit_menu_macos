use crate::{key, menu_item::make_menu_item};
use cocoa::{
  appkit::{NSEventModifierFlags, NSMenuItem},
  base::{id, nil, selector},
};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};

/// A menu item, bound to a pre-defined native action.
///
/// Note some platforms might not support some of the variants.
/// Unsupported variant will be no-op on such platform.
///
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum NativeMenuItem {
  About(String),
  CloseWindow,
  Separator,
  Quit,
  Hide,
  HideOthers,
  ShowAll,
  EnterFullScreen,
  Minimize,
  Zoom,
  Copy,
  Cut,
  Paste,
  Undo,
  Redo,
  SelectAll,
  Services,
}

pub fn make_native_menu_item(
  item: NativeMenuItem,
  title: Option<&str>,
  key_equivalent: Option<key::KeyEquivalent>,
) -> *mut Object {
  let (_, menu_item) = match item {
    NativeMenuItem::Separator => unsafe { (None, NSMenuItem::separatorItem(nil)) },
    NativeMenuItem::About(app_name) => {
      let _title = format!("About {}", app_name);
      make_menu_item(
        title.unwrap_or(_title.as_str()),
        Some(selector("orderFrontStandardAboutPanel:")),
        key_equivalent,
      )
    }
    NativeMenuItem::CloseWindow => make_menu_item(
      title.unwrap_or("Close Window"),
      Some(selector("performClose:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "w",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Quit => make_menu_item(
      title.unwrap_or("Quit"),
      Some(selector("terminate:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "q",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Hide => make_menu_item(
      title.unwrap_or("Hide"),
      Some(selector("hide:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "h",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::HideOthers => make_menu_item(
      title.unwrap_or("Hide Others"),
      Some(selector("hideOtherApplications:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "h",
        masks: Some(NSEventModifierFlags::NSAlternateKeyMask),
      })),
    ),
    NativeMenuItem::ShowAll => make_menu_item(
      title.unwrap_or("Show All"),
      Some(selector("unhideAllApplications:")),
      None,
    ),
    NativeMenuItem::EnterFullScreen => make_menu_item(
      title.unwrap_or("Enter Full Screen"),
      Some(selector("toggleFullScreen:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "f",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Minimize => make_menu_item(
      title.unwrap_or("Minimize"),
      Some(selector("performMiniaturize:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "m",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Zoom => make_menu_item(
      title.unwrap_or("Zoom"),
      Some(selector("performZoom:")),
      None,
    ),
    NativeMenuItem::Copy => make_menu_item(
      title.unwrap_or("Copy"),
      Some(selector("copy:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "c",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Cut => make_menu_item(
      title.unwrap_or("Cut"),
      Some(selector("cut:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "x",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Paste => make_menu_item(
      title.unwrap_or("Paste"),
      Some(selector("paste:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "v",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Undo => make_menu_item(
      title.unwrap_or("Undo"),
      Some(selector("undo:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "z",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Redo => make_menu_item(
      title.unwrap_or("Redo"),
      Some(selector("redo:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "z",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask | NSEventModifierFlags::NSShiftKeyMask),
      })),
    ),
    NativeMenuItem::SelectAll => make_menu_item(
      title.unwrap_or("Select All"),
      Some(selector("selectAll:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "a",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
    ),
    NativeMenuItem::Services => unsafe {
      let (_, item) = make_menu_item("Services", None, key_equivalent);
      let app_class = class!(NSApplication);
      let app: id = msg_send![app_class, sharedApplication];
      let services: id = msg_send![app, servicesMenu];
      let _: () = msg_send![&*item, setSubmenu: services];
      (None, item)
    },
  };

  menu_item
}
