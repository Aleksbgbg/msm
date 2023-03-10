use tauri::State;

use crate::types::app_data::AppData;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstTimeSetup {
  required: bool,
  has_java: bool,
}

#[tauri::command]
pub fn requires_first_time_setup(app_data: State<AppData>) -> FirstTimeSetup {
  FirstTimeSetup {
    required: true,  // completed_first_time_setup(&app_data.settings),
    has_java: false, // env::var("PATH").unwrap().contains("Java"),
  }
}
