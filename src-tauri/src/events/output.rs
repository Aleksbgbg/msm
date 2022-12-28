use tauri::Window;

use crate::types::result::Result;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
  pub line: String,
}

const EVENT_OUTPUT: &str = "output";

pub fn send_output(window: &Window, payload: Payload) -> Result<()> {
  window.emit(EVENT_OUTPUT, payload)?;
  Ok(())
}
