// use cocoa::base::id;
use crate::{
  menu::MenuId,
  menu_item::{Id, MenuType},
};

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Event {
  MenuEvent {
    window_id: Option<Id>,
    menu_id: MenuId,
    menu_type: MenuType,
  },
}
