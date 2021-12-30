//! Add native shadows to your Tauri/TAO windows.
//!
//! # Platforms Note:
//!
//! - Only Windows and macOS are supported,
//! Linux shadows are controlled by the compositor installed on the user system and they can enable it for your app if they want.
//! - On Windows, shadows can't be turned off on a regular(decorated) window.
//!
//! # Usage:
//!
//! Import the [`Shadows`] trait and use [`Shadows::set_shadow()`] on your window type
//! - Tauri:
//!   ```ignore
//!   let window = app.get_window("main").unwrap();
//!   use tauri_plugin_shadows::Shadows;
//!   window.set_shadow(true);
//!   ```
//! - Tao:
//!   ```ignore
//!   let window = WindowBuilder::new().with_transparent(true).build(&event_loop).unwrap();
//!   use tauri_plugin_shadows::Shadows;
//!   window.set_shadow(true);
//!   ```

#![allow(unused)]

mod platform;

#[cfg(target_os = "macos")]
use crate::platform::macos;
#[cfg(target_os = "windows")]
use crate::platform::windows;

#[cfg(feature = "tauri-impl")]
use tauri::{Runtime, Window as TauriWindow};

#[cfg(all(target_os = "macos", feature = "tao-impl"))]
use tao::platform::macos::WindowExtMacOS;
#[cfg(all(target_os = "windows", feature = "tao-impl"))]
use tao::platform::windows::WindowExtWindows;
#[cfg(feature = "tao-impl")]
use tao::window::Window as TaoWindow;

pub trait Shadows {
  /// Sets the shadows on the window.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows:** shadows can't be turned off on a regular(decorated) window.
  fn set_shadow(&self, shadow: bool);
}

#[cfg(feature = "tauri-impl")]
impl<R> Shadows for TauriWindow<R>
where
  R: Runtime,
{
  fn set_shadow(&self, shadow: bool) {
    #[cfg(all(target_os = "windows", feature = "tauri-impl"))]
    windows::set_shadow(self.hwnd().unwrap() as _, shadow);
    #[cfg(all(target_os = "macos", feature = "tauri-impl"))]
    macos::set_shadow(self.ns_window().unwrap() as _, shadow);
  }
}

#[cfg(feature = "tao-impl")]
impl Shadows for TaoWindow {
  fn set_shadow(&self, shadow: bool) {
    #[cfg(all(target_os = "windows", feature = "tao-impl"))]
    windows::set_shadow(self.hwnd() as _, shadow);
    #[cfg(all(target_os = "macos", feature = "tao-impl"))]
    self.set_has_shadow(shadow);
  }
}
