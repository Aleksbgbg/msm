#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserData {
  worlds: Vec<World>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct World {
  name: String,
  description: String,
  //saves: Vec<Save>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Save {
  // #[serde(with = "ts_seconds_option")]
  // save_time: DateTime<Utc>,
}

impl Default for UserData {
  fn default() -> Self {
    Self { worlds: Vec::new() }
  }
}
