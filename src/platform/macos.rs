// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(target_os = "macos")]

use cocoa::{appkit::NSWindow, base::id};
use objc::runtime::{NO, YES};

pub fn set_shadow(window: id, shadow: bool) {
  unsafe {
    window.setHasShadow_(if shadow { YES } else { NO });
  }
}
