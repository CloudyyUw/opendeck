#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, Menu, Submenu};

fn main() {
  let server_menu = Submenu::new(
    "Server",
    Menu::with_items([CustomMenuItem::new("Start Server", "Start Server").into()]),
  );

  tauri::Builder::default()
    .menu(Menu::new().add_submenu(server_menu))
    .on_menu_event(|event| match event.menu_item_id() {
      "Start Server" => {}
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
