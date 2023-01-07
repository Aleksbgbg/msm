#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;
mod events;
mod setup;
mod types;

use std::env;

use tauri::{generate_handler, Builder, Manager};
use types::program_data::ProgramData;

use crate::commands::first_time_setup::requires_first_time_setup;
use crate::types::result::Result;

#[tokio::main]
async fn main() -> Result<()> {
  Builder::default()
    .manage(ProgramData::default())
    .invoke_handler(generate_handler![requires_first_time_setup,])
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      main_window.open_devtools();

      Ok(())
    })
    .run(tauri::generate_context!())?;

  Ok(())
}
