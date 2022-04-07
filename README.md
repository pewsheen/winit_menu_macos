# winit_menu_macos

**MacOS** menu module for [winit](https://github.com/rust-windowing/winit)

---

Credit goes to [tao](https://github.com/tauri-apps/tao) and [winit](https://github.com/rust-windowing/winit).

This crate modularized menu related mods from [tao](https://github.com/tauri-apps/tao), so you can depend this mod only when you needed instead of the whole tao library.

Due to there is no simple way to inject WindowEvent in winit, we use mpsc to send click event.

See [examples](https://github.com/pewsheen/winit_menu_macos/tree/main/examples) to learn how to set up menu and menu item.


