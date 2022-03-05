# window-shadows

[![](https://img.shields.io/crates/v/window-shadows)](https://crates.io/crates/window-shadows) [![](https://img.shields.io/docsrs/window-shadows)](https://docs.rs/window-shadows/) ![](https://img.shields.io/crates/l/window-shadows)
[![Chat Server](https://img.shields.io/badge/chat-on%20discord-7289da.svg)](https://discord.gg/SpmNs4S)

Add native shadows to your windows.

## Platform support

 - **`Windows:`** Yes, but shadows can't be turned off for a normal (decorated) window.
 - **`macOS:`** Yes!
 - **`Linux:`** No, shadows are controlled by the compositor installed on the user system and they can enable it for your app if they want.

## Examples

- with `winit`:
    ```rs
    use winit::{event_loop::EventLoop, window::WindowBuilder};
    use window_shadows::set_shadow

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
    .with_decorations(false)
    .build(&event_loop)
    .unwrap();

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    set_shadow(&window, true).unwrap();
    ```

- with `tauri`:
    ```rs
    use window_shadows::set_shadow

    let window = app.get_window("main").unwrap();

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    set_shadow(&window, true).unwrap();
    ```
