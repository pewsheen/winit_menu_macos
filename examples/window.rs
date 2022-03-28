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

  let menu_bar = menu_bartender();

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

fn menu_bartender() -> Menu {
  /* menu bar */
  let menu_bar: Menu = Menu::new();

  /* application menu */
  let app_menu: Menu = Menu::new();
  app_menu.add_native_item(NativeMenuItem::About("AppName".to_string()), None, None);
  app_menu.add_item("AppMenu Item 1", None, None);
  app_menu.add_item("AppMenu Item 2", None, None);
  app_menu.add_item("AppMenu Item 3", None, None);
  app_menu.add_native_item(NativeMenuItem::HideOthers, None, None);
  app_menu.add_native_item(NativeMenuItem::Separator, None, None);
  app_menu.add_native_item(NativeMenuItem::CloseWindow, Some("Bye"), None);

  menu_bar.add_submenu(&app_menu, "Application");

  /* first menu */
  let first_menu: Menu = Menu::new();
  first_menu.add_item(
    "Menu Item A",
    None,
    Some(KeyEquivalent {
      key: "h",
      masks: Some(
        NSEventModifierFlags::NSAlternateKeyMask | NSEventModifierFlags::NSCommandKeyMask,
      ),
    }),
  );
  first_menu.add_item("Menu Item B", None, None);
  first_menu.add_item("Menu Item C", None, None);
  first_menu.add_item("Menu Item D", None, None);

  menu_bar.add_submenu(&first_menu, "First Menu");

  menu_bar
}
