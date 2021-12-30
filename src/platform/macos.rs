#![cfg(target_os = "macos")]

use cocoa::{appkit::NSWindow, base::id};
use objc::runtime::{NO, YES};

pub fn set_shadow(window: id, shadow: bool) {
  window.setHasShadow_(if shadow { YES } else { NO })
}
