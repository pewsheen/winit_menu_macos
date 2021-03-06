use crate::platform_impl::{key, menu::MenuType, menu_item::make_menu_item};
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
pub enum NativeMenuItemType {
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
  item: NativeMenuItemType,
  title: Option<&str>,
  key_equivalent: Option<key::KeyEquivalent>,
  menu_type: MenuType,
) -> *mut Object {
  let (_, menu_item) = match item {
    NativeMenuItemType::Separator => unsafe { (None, NSMenuItem::separatorItem(nil)) },
    NativeMenuItemType::About(app_name) => {
      let _title = format!("About {}", app_name);
      make_menu_item(
        title.unwrap_or(_title.as_str()),
        Some(selector("orderFrontStandardAboutPanel:")),
        key_equivalent,
        menu_type,
      )
    }
    NativeMenuItemType::CloseWindow => make_menu_item(
      title.unwrap_or("Close Window"),
      Some(selector("performClose:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "w",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Quit => make_menu_item(
      title.unwrap_or("Quit"),
      Some(selector("terminate:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "q",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Hide => make_menu_item(
      title.unwrap_or("Hide"),
      Some(selector("hide:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "h",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::HideOthers => make_menu_item(
      title.unwrap_or("Hide Others"),
      Some(selector("hideOtherApplications:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "h",
        masks: Some(NSEventModifierFlags::NSAlternateKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::ShowAll => make_menu_item(
      title.unwrap_or("Show All"),
      Some(selector("unhideAllApplications:")),
      None,
      menu_type,
    ),
    NativeMenuItemType::EnterFullScreen => make_menu_item(
      title.unwrap_or("Enter Full Screen"),
      Some(selector("toggleFullScreen:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "f",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Minimize => make_menu_item(
      title.unwrap_or("Minimize"),
      Some(selector("performMiniaturize:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "m",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Zoom => make_menu_item(
      title.unwrap_or("Zoom"),
      Some(selector("performZoom:")),
      None,
      menu_type,
    ),
    NativeMenuItemType::Copy => make_menu_item(
      title.unwrap_or("Copy"),
      Some(selector("copy:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "c",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Cut => make_menu_item(
      title.unwrap_or("Cut"),
      Some(selector("cut:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "x",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Paste => make_menu_item(
      title.unwrap_or("Paste"),
      Some(selector("paste:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "v",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Undo => make_menu_item(
      title.unwrap_or("Undo"),
      Some(selector("undo:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "z",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Redo => make_menu_item(
      title.unwrap_or("Redo"),
      Some(selector("redo:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "z",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask | NSEventModifierFlags::NSShiftKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::SelectAll => make_menu_item(
      title.unwrap_or("Select All"),
      Some(selector("selectAll:")),
      Some(key_equivalent.unwrap_or(key::KeyEquivalent {
        key: "a",
        masks: Some(NSEventModifierFlags::NSCommandKeyMask),
      })),
      menu_type,
    ),
    NativeMenuItemType::Services => unsafe {
      let (_, item) = make_menu_item("Services", None, key_equivalent, menu_type);
      let app_class = class!(NSApplication);
      let app: id = msg_send![app_class, sharedApplication];
      let services: id = msg_send![app, servicesMenu];
      let _: () = msg_send![&*item, setSubmenu: services];
      (None, item)
    },
  };

  menu_item
}
