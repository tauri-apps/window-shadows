# tauri-plugin-shadows

Add native shadows to your Tauri/TAO windows.

## Platform support

 - **`Windows:`** Yes, but shadows can't be turned off for a normal (decorated) window.
 - **`macOS:`** Yes!
 - **`Linux:`** No, shadows are controlled by the compositor installed on the user system and they can enable it for your app if they want.

## Installation

Add it as a dependncy in `Cargo.toml` of your Tao/Tauri project
```toml
[dependencies]
tauri-plugin-shadows = { git = "https://github.com/tauri-apps/tauri-plugin-shadows", features = ["tauri-impl"] } # or "tao-impl" for TAO projects.
```

## Cargo Features:

- `tauri-impl`: for Tauri projects.
- `tao-impl`: for TAO projects.

## Usage
Import the `Shadows` trait and use `set_shadow()` on your window type:
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