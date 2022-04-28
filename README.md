# window-shadows

[![](https://img.shields.io/crates/v/window-shadows)](https://crates.io/crates/window-shadows) [![](https://img.shields.io/docsrs/window-shadows)](https://docs.rs/window-shadows/) ![](https://img.shields.io/crates/l/window-shadows)
[![Chat Server](https://img.shields.io/badge/chat-on%20discord-7289da.svg)](https://discord.gg/SpmNs4S)

Add native shadows to your windows.

## Platform-specific

- **Windows**: On Windows 11, the window will also have rounded corners.
- **Linux**: Unsupported, Shadows are controlled by the compositor installed on the end-user system.

## Example

```rs
use window_shadows::set_shadow;

set_shadow(&window, true).expect("Unsupported platform!");
```
