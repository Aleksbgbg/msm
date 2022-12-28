#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;
mod events;
mod settings;
mod setup;
mod structs;
mod types;

use std::env;
use std::io::BufRead;
use std::process::Command;

use events::output::{send_output, Payload};
use settings::load_settings;
use tauri::{generate_handler, Builder, Manager};
use types::app_data::AppData;

use crate::commands::first_time_setup::requires_first_time_setup;
use crate::types::result::Result;

#[tokio::main]
async fn main() -> Result<()> {
  Builder::default()
    .manage(AppData {
      settings: load_settings()?,
    })
    .invoke_handler(generate_handler![requires_first_time_setup,])
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      main_window.open_devtools();

      Ok(())
    })
    .run(tauri::generate_context!())?;

  Ok(())
}
