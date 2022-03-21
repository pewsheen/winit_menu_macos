use winit::{
  event::{DeviceEvent, ElementState, Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

use winit_menu_macos::menu::{set_menu, Menu, MenuItem};

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

  let app_menu_item_1: MenuItem = MenuItem::new("AppMenu Item 1", None);
  let app_menu_item_2: MenuItem = MenuItem::new("AppMenu Item 2", None);
  let app_menu_item_3: MenuItem = MenuItem::new("AppMenu Item 3", None);

  app_menu.add_item(&app_menu_item_1);
  app_menu.add_item(&app_menu_item_2);
  app_menu.add_item(&app_menu_item_3);

  menu_bar.add_submenu(&app_menu, "Application");

  /* first menu */
  let first_menu: Menu = Menu::new();

  let first_menu_item_a = MenuItem::new("Menu Item A", None);
  let first_menu_item_b = MenuItem::new("Menu Item B", None);
  let first_menu_item_c = MenuItem::new("Menu Item C", None);
  let first_menu_item_d = MenuItem::new("Menu Item D", None);

  first_menu.add_item(&first_menu_item_a);
  first_menu.add_item(&first_menu_item_b);
  first_menu.add_item(&first_menu_item_c);
  first_menu.add_item(&first_menu_item_d);

  menu_bar.add_submenu(&first_menu, "First Menu");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id,
      } if window_id == window.id() => *control_flow = ControlFlow::Exit,
      Event::MainEventsCleared => {
        window.request_redraw();
      }
      Event::DeviceEvent { event, .. } => match event {
        DeviceEvent::Button { button: _, state } => match state {
          ElementState::Released => {
            set_menu(&menu_bar);
          }
          _ => (),
        },
        _ => (),
      },
      _ => (),
    }
  });
}
