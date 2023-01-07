use crate::types::result::Result;

#[derive(Debug, Eq, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct MinecraftVersion {
  pub major: u32,
  pub minor: u32,
  pub release: u32,
}

impl MinecraftVersion {
  pub fn from_string(string: &String) -> Result<MinecraftVersion> {
    let split: Vec<&str> = string.split(".").collect();
    Ok(MinecraftVersion {
      major: split[0].parse()?,
      minor: split[1].parse()?,
      release: split[2].parse()?,
    })
  }
}
