# tauri-plugin-shadows

Add native shadows to your Tauri/TAO windows.

## Platforms Notes

- Only Windows and macOS are supported,
Linux shadows are controlled by the compositor installed on the user system and they can enable it for your app if they want.
- On Windows, shadows can't be turned off on a regular(decorated) window.

## Installation

Add it as a dependncy in `Cargo.toml` of your Tao/Tauri project
```toml
[dependencies]
tauri-plugin-shadows = { git = "https://github.com/tauri-apps/tauri-plugin-shadows", features = ["tauri-impl"] }
```
You also need to use Tauri/TAO from github using the `next` branch (Only until the next release of Tauri).

## Crate Features:

- `tauri-impl`: for Tauri projects.
- `tao-impl`: for TAO projects.

## Usage
Import the `Shadows` trait and use `set_shadow()` on your window type
- Tauri:
    ```rs
    let window = app.get_window("main").unwrap();

    use tauri_plugin_shadows::Shadows;
    window.set_shadow(true);
    ```
- Tao:
    ```rs
    let window = WindowBuilder::new().with_transparent(true).build(&event_loop).unwrap();

    use tauri_plugin_shadows::Shadows;
    window.set_shadow(true);
    ```