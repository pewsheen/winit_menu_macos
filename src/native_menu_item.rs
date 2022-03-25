use cocoa::{
  appkit::{NSEventModifierFlags, NSMenuItem},
  base::{nil, selector},
};
use objc::runtime::Object;

use crate::{key, menu_item::make_menu_item};

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
  };

  menu_item
}
