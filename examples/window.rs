use cocoa::appkit::NSEventModifierFlags;
use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

use winit_menu_macos::{
  event_channel::get_event_channel,
  key::KeyEquivalent,
  menu::{set_menu, Menu},
  menu_item::MenuItem,
  native_menu_item::NativeMenuItem,
};

fn main() {
  let event_loop = EventLoop::new();

  let window = WindowBuilder::new()
    .with_title("A fantastic window!")
    .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
    .build(&event_loop)
    .unwrap();

  /* menu bar */
  let menu_bar: Menu = Menu::new();

  /* application menu */
  let app_menu: Menu = Menu::new();

  let app_menu_item_1: MenuItem = MenuItem::new("AppMenu Item 1", None, None);
  let app_menu_item_2: MenuItem = MenuItem::new("AppMenu Item 2", None, None);
  let app_menu_item_3: MenuItem = MenuItem::new("AppMenu Item 3", None, None);

  /* native menu items */
  let about_menu_item =
    MenuItem::new_native(NativeMenuItem::About("AppName".to_string()), None, None);
  let sep: MenuItem = MenuItem::new_native(NativeMenuItem::Separator, None, None);
  let close_menu_item: MenuItem =
    MenuItem::new_native(NativeMenuItem::CloseWindow, Some("Bye"), None);

  app_menu.add_item(&about_menu_item);
  app_menu.add_item(&app_menu_item_1);
  app_menu.add_item(&app_menu_item_2);
  app_menu.add_item(&app_menu_item_3);
  app_menu.add_item(&sep);
  app_menu.add_item(&close_menu_item);

  menu_bar.add_submenu(&app_menu, "Application");

  /* first menu */
  let first_menu: Menu = Menu::new();

  let first_menu_item_a = MenuItem::new(
    "Menu Item A",
    None,
    Some(KeyEquivalent {
      key: "h",
      masks: Some(
        NSEventModifierFlags::NSAlternateKeyMask | NSEventModifierFlags::NSCommandKeyMask,
      ),
    }),
  );
  let first_menu_item_b = MenuItem::new("Menu Item B", None, None);
  let first_menu_item_c = MenuItem::new("Menu Item C", None, None);
  let first_menu_item_d = MenuItem::new("Menu Item D", None, None);

  first_menu.add_item(&first_menu_item_a);
  first_menu.add_item(&first_menu_item_b);
  first_menu.add_item(&first_menu_item_c);
  first_menu.add_item(&first_menu_item_d);

  menu_bar.add_submenu(&first_menu, "First Menu");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    let channel = get_event_channel();
    let rx_ref = channel.1.clone();

    /* recv menu events */
    while let Ok(data) = rx_ref.try_recv() {
      println!("{:?}", data);
    }

    match event {
      Event::NewEvents(winit::event::StartCause::Init) => set_menu(&menu_bar),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id,
      } if window_id == window.id() => *control_flow = ControlFlow::Exit,
      Event::MainEventsCleared => {
        window.request_redraw();
      }
      _ => (),
    }
  });
}
