use super::minecraft_version::MinecraftVersion;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProgramData {
  // True if first time setup has completed
  #[serde(default)]
  first_time_setup_complete: bool,

  // Version of Minecraft server currently stored.
  #[serde(default)]
  minecraft_version: Option<MinecraftVersion>,
}

impl Default for ProgramData {
  fn default() -> Self {
    Self {
      first_time_setup_complete: false,
      minecraft_version: None,
    }
  }
}
