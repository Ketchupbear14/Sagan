// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Also in main.rs
fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
