use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::settings;
use crate::settings::Settings;
use crate::structs::minecraft_version::MinecraftVersion;
use crate::types::result::Result;

const SERVER_WEBPAGE_URL: &str = "https://www.minecraft.net/en-us/download/server";
const SERVER_INFO_REGEX: &str = r#"<a href="(.*)" aria-label="mincraft version">minecraft_server\.(.*)\.jar</a>"#;
const SERVER_FILENAME: &str = "minecraft_server.jar";

struct MinecraftServerInfo {
  version: MinecraftVersion,
  download_url: String,
}

fn get_server_info(server_webpage: &String) -> Result<MinecraftServerInfo> {
  let server_info_matches = Regex::new(SERVER_INFO_REGEX)?
    .captures(&server_webpage)
    .ok_or("No regex result")?;

  Ok(MinecraftServerInfo {
    version: MinecraftVersion::from_string(&server_info_matches.get(2).ok_or("No match 2")?.as_str().to_string())?,
    download_url: server_info_matches.get(1).ok_or("No match 1")?.as_str().to_string(),
  })
}

pub async fn download_latest_server(folder: &String, settings: &mut Settings) -> Result<()> {
  let mut headers = HeaderMap::new();
  headers.insert(
    "Accept",
    HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
  );
  headers.insert("Accept-Language", HeaderValue::from_static("en-GB,en;q=0.5"));
  headers.insert("Accept-Encoding", HeaderValue::from_static("identity"));
  headers.insert("DNT", HeaderValue::from_static("1"));
  headers.insert("Connection", HeaderValue::from_static("keep-alive"));
  headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
  headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
  headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
  headers.insert("Sec-Fetch-Site", HeaderValue::from_static("cross-site"));
  headers.insert("TE", HeaderValue::from_static("trailers"));
  headers.insert(
    "User-Agent",
    HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:108.0) Gecko/20100101 Firefox/108.0"),
  );

  let client = Client::builder().default_headers(headers).build()?;

  let server_webpage = client.get(SERVER_WEBPAGE_URL).send().await?.text().await?;

  let server_info = get_server_info(&server_webpage)?;

  if !settings::is_latest_minecraft_version_with_update(settings, server_info.version) {
    let mut server_file = File::create(Path::new(folder).join(SERVER_FILENAME))?;
    std::io::copy(
      &mut Cursor::new(client.get(server_info.download_url).send().await?.bytes().await?),
      &mut server_file,
    )?;
  }

  Ok(())
}
