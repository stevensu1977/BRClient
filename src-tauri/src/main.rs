// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod context_file;

use std::path::PathBuf;

#[tauri::command(rename_all = "snake_case")]
fn tauri_file_extract_command(invoke_message: String)  -> String {
  println!("I was invoked from JS, with this message: {}",invoke_message);
  
  let result = context_file::read(PathBuf::from(invoke_message));
    match result {
      Ok(contents) => {
        println!("File contents retrieved successfully, {}.",contents);
        contents
    },
    Err(err) => {
        let error_message = format!("Error reading file: {}", err);
        eprintln!("{}", error_message);
        error_message
    },
    }
}



fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![tauri_file_extract_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
