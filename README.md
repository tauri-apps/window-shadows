# window-shadows

Add native shadows to your windows.

## Platform support

 - **`Windows:`** Yes, but shadows can't be turned off for a normal (decorated) window.
 - **`macOS:`** Yes!
 - **`Linux:`** No, shadows are controlled by the compositor installed on the user system and they can enable it for your app if they want.

## Installation

Add it as a dependncy in `Cargo.toml` of your Tao/Tauri project
```toml
[dependencies]
window-shadows = { git = "https://github.com/tauri-apps/window-shadows" }
```

## Examples

- with `winit`:
    ```rs
    use winit::{event_loop::EventLoop, window::WindowBuilder};
    use window_shadows::set_shadow

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
    .with_decorations(false)
    .with_transparent(true)
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
