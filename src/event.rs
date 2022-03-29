use crate::platform_impl::menu::{Id, MenuId, MenuType};

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Event {
  MenuEvent {
    window_id: Option<Id>,
    menu_id: MenuId,
    menu_type: MenuType,
  },
}
