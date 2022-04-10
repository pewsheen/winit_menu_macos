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
  platform_impl::{
    key::KeyEquivalent, menu::MenuId, menu_item_attributes::MenuItemAttributes,
    native_menu_item_type::NativeMenuItemType,
  },
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
    NativeMenuItemType::About("AppName".to_string()),
    None,
    Some(KeyEquivalent {
      key: "a",
      masks: Some(NSEventModifierFlags::NSControlKeyMask | NSEventModifierFlags::NSCommandKeyMask),
    }),
  );
  // create cumtom menu item with add_item.
  // add_item required MenuItemAttributes to setup initial state.
  // `title` is required, `selector` and `key equivalent` are optional.
  // `enabled` defult to true, `selected` defult to false.
  app_menu.add_item(MenuItemAttributes::new("AppMenu Item 1"));
  app_menu.add_item(MenuItemAttributes::new("AppMenu Item 2"));
  app_menu.add_item(MenuItemAttributes::new("AppMenu Item 3"));
  app_menu.add_native_item(NativeMenuItemType::HideOthers, None, None);
  app_menu.add_native_item(NativeMenuItemType::Separator, None, None);
  app_menu.add_native_item(NativeMenuItemType::CloseWindow, Some("Bye"), None);

  // We can't change the title of the window in macOS, so the title here is useless
  menu_bar.add_submenu(&app_menu, "Application");

  /* first menu */
  let first_menu: Menu = Menu::new();

  // Save the item id for later use
  // We will make this item disabled after click event triggered
  let mut enable_test_item = first_menu.add_item(
    MenuItemAttributes::new("Click to disable this item").with_key_equivalent(KeyEquivalent {
      key: "h",
      masks: Some(
        NSEventModifierFlags::NSAlternateKeyMask | NSEventModifierFlags::NSCommandKeyMask,
      ),
    }),
  );
  first_menu.add_item(MenuItemAttributes::new("Menu Item B"));
  first_menu.add_item(
    MenuItemAttributes::new("Menu Item C")
      .with_enabled(false)
      .with_selected(true),
  );
  first_menu.add_item(MenuItemAttributes::new("Menu Item D").with_enabled(false));

  menu_bar.add_submenu(&first_menu, "First Menu");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    // Get menu event recv channel
    let channel = get_event_channel();
    let rx_ref = channel.1.clone();

    // Here we get menu click events through the channel
    while let Ok(event) = rx_ref.try_recv() {
      println!("{:?}", event);

      match event {
        MenuEvent { menu_id, .. } => {
          if menu_id == MenuId::EMPTY {
            // menu item without title will generate a empty Id and trapped here
            println!("EMPTY menu id");
          } else if menu_id == enable_test_item.id() {
            // use MenuItem.id() to get item id and match the event
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
