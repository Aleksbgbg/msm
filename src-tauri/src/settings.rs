use std::fs::File;
use std::io::{Read, Write};

use crate::structs::minecraft_version::MinecraftVersion;
use crate::types::result::Result;

const SETTINGS_FILENAME: &str = "settings";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ProgramData {
  // True if first time setup has completed
  #[serde(default)]
  first_time_setup_complete: bool,

  // Version of Minecraft server currently stored.
  #[serde(default)]
  minecraft_version: Option<MinecraftVersion>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UserSettings {
  // The working directory of the Minecraft server. When not present, the directory will be created in the OS' temp
  // folder and deleted when the server is shut down, after the world is saved.
  #[serde(default)]
  server_directory: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Save {
  // #[serde(with = "ts_seconds_option")]
  // save_time: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct World {
  name: String,
  description: String,
  //saves: Vec<Save>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UserData {
  worlds: Vec<World>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
  program_data: ProgramData,
  user_settings: UserSettings,
  user_data: UserData,
}

fn default_settings() -> Settings {
  Settings {
    program_data: ProgramData {
      first_time_setup_complete: false,
      minecraft_version: None,
    },
    user_settings: UserSettings { server_directory: None },
    user_data: UserData { worlds: Vec::new() },
  }
}

fn write_settings(settings: &Settings, file: &mut File) -> Result<()> {
  file.write_all(serde_json::to_string(settings)?.as_bytes())?;
  Ok(())
}

pub fn load_settings() -> Result<Settings> {
  let mut file = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open(SETTINGS_FILENAME)?;

  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  if contents.len() == 0 {
    let default_settings = default_settings();
    write_settings(&default_settings, &mut file)?;
    Ok(default_settings)
  } else {
    Ok(serde_json::from_str::<Settings>(&contents)?)
  }
}

pub fn save_settings(settings: &Settings) -> Result<()> {
  write_settings(settings, &mut File::options().write(true).open(SETTINGS_FILENAME)?)?;
  Ok(())
}

pub fn completed_first_time_setup(settings: &Settings) -> bool {
  settings.program_data.first_time_setup_complete
}

pub fn is_latest_minecraft_version_with_update(settings: &mut Settings, latest_version: MinecraftVersion) -> bool {
  if settings.program_data.minecraft_version == Some(latest_version) {
    true
  } else {
    settings.program_data.minecraft_version = Some(latest_version);
    false
  }
}
