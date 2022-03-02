// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Add native shadows to your windows.
//!
//! # Platform support:
//!
//! - **Windows:** Yes, but shadows can't be turned off for a normal (decorated) window.
//! - **macOS:** Yes!
//! - **Linux:** No, shadows are controlled by the compositor installed on the user system and they can enable it for your app if they want.
//!
//! # Example with [`winit`](https://docs.rs/winit)
//!
//! ```no_run
//! # use winit::{event_loop::EventLoop, window::WindowBuilder};
//! # use window_shadows::set_shadow;
//! let event_loop = EventLoop::new();
//!
//! let window = WindowBuilder::new()
//!  .with_decorations(false)
//!  .with_transparent(true)
//!  .build(&event_loop)
//!  .unwrap();
//!
//! #[cfg(any(target_os = "windows", target_os = "macos"))]
//! set_shadow(&window, true).unwrap();
//! ```

/// Enables or disables the shadows for a window.
pub fn set_shadow(
  window: impl raw_window_handle::HasRawWindowHandle,
  enable: bool,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "macos")]
    raw_window_handle::RawWindowHandle::AppKit(handle) => {
      use cocoa::{appkit::NSWindow, base::id};
      use objc::runtime::{NO, YES};

      unsafe {
        handle
          .ns_window
          .setHasShadow_(if enable { YES } else { NO });
      }

      Ok(())
    }
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      use windows_sys::Win32::{
        Graphics::Dwm::DwmExtendFrameIntoClientArea, UI::Controls::MARGINS,
      };

      let m = if enable { 1 } else { 0 };
      let margins = MARGINS {
        cxLeftWidth: m,
        cxRightWidth: m,
        cyTopHeight: m,
        cyBottomHeight: m,
      };
      unsafe {
        DwmExtendFrameIntoClientArea(handle.hwnd as _, &margins);
      };
      Ok(())
    }
    _ => Err(Error::UnsupportedPlatform),
  }
}

#[derive(Debug)]
pub enum Error {
  UnsupportedPlatform,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\"set_shadow()\" is only supported on Windows and macOS")
  }
}
