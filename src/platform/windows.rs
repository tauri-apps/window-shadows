// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(target_os = "windows")]

pub use windows::Win32::{
  Foundation::HWND, Graphics::Dwm::DwmExtendFrameIntoClientArea, UI::Controls::MARGINS,
};

pub fn set_shadow(hwnd: HWND, shadow: bool) {
  let m = if shadow { 1 } else { 0 };
  let margins = MARGINS {
    cxLeftWidth: m,
    cxRightWidth: m,
    cyTopHeight: m,
    cyBottomHeight: m,
  };
  unsafe {
    DwmExtendFrameIntoClientArea(hwnd, &margins);
  }
}
