#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_shadows::Shadows;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      window.set_shadow(true);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
