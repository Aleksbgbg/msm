use super::program_data::ProgramData;
use super::result::Result;
use super::user_data::UserData;
// use super::user_settings::UserSettings;

pub struct AppData {
  pub program_data: ProgramData,
  //pub user_settings: UserSettings,
  pub user_data: UserData,
}

impl Default for AppData {
  fn default() -> Self {
    Self {
      program_data: ProgramData::default(),
      //user_settings: UserSettings::default(),
      user_data: UserData::default(),
    }
  }
}

impl AppData {
  pub fn load() -> Result<Self> {
    Ok(Self::default())
  }
}
