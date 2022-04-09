use cocoa::appkit::NSEventModifierFlags;
use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use winit_menu_macos::{
  event::Event::MenuEvent,
  event_channel::get_event_channel,
  menu::{set_menu, Menu},
  platform_impl::{key::KeyEquivalent, menu::MenuId, native_menu_item::NativeMenuItem},
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

  // (ctrl + cmd + a) to open about window
  app_menu.add_native_item(
    NativeMenuItem::About("AppName".to_string()),
    None,
    Some(KeyEquivalent {
      key: "a",
      masks: Some(NSEventModifierFlags::NSControlKeyMask | NSEventModifierFlags::NSCommandKeyMask),
    }),
  );
  app_menu.add_item("AppMenu Item 1", None, None, None, None);
  app_menu.add_item("AppMenu Item 2", None, None, None, None);
  app_menu.add_item("AppMenu Item 3", None, None, None, None);
  app_menu.add_native_item(NativeMenuItem::HideOthers, None, None);
  app_menu.add_native_item(NativeMenuItem::Separator, None, None);
  app_menu.add_native_item(NativeMenuItem::CloseWindow, Some("Bye"), None);

  menu_bar.add_submenu(&app_menu, "Application");

  /* first menu */
  let first_menu: Menu = Menu::new();
  let mut enable_test_item = first_menu.add_item(
    "Click to disable this item",
    None,
    Some(KeyEquivalent {
      key: "h",
      masks: Some(
        NSEventModifierFlags::NSAlternateKeyMask | NSEventModifierFlags::NSCommandKeyMask,
      ),
    }),
    None,
    None,
  );
  first_menu.add_item("Menu Item B", None, None, None, None);
  first_menu.add_item("Menu Item C", None, None, Some(false), Some(true));
  first_menu.add_item("Menu Item D", None, None, Some(false), Some(false));

  menu_bar.add_submenu(&first_menu, "First Menu");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    let channel = get_event_channel();
    let rx_ref = channel.1.clone();

    /* recv menu events */
    while let Ok(event) = rx_ref.try_recv() {
      println!("{:?}", event);

      match event {
        MenuEvent { menu_id, .. } => {
          if menu_id == MenuId::EMPTY {
            println!("EMPTY menu id");
          } else if menu_id == enable_test_item.id() {
            enable_test_item.set_enabled(false);
          }
        }
        _ => (),
      }
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
