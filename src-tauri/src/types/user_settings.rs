use std::collections::HashMap;

// type DynamicValue = Box<dyn Value>;

struct Definition<T: Value + ?Sized> {
  key: Key,
  name: &'static str,
  description: &'static str,
  optional: bool,
  data_type: DataType,
  input_type: InputType,
  default_value: Box<T>,
}

enum DataType {
  String,
}

enum InputType {
  PathFolder,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Setting {
  AppDataDirectory,
  ServerDirectory,
}

trait Value : Clone /* : Send + serde::Serialize + serde::Deserialize<'de> */ {}

#[derive(serde::Serialize, serde::Deserialize)]
struct StringValue {
  contents: String,
}

impl StringValue {
  fn from(contents: String) -> Self {
    Self { contents }
  }
}

impl Value for StringValue {}

#[derive(serde::Serialize, serde::Deserialize)]
struct OptionalValue<T>
where
  T: Value,
{
  contents: Option<T>,
}

impl<T> OptionalValue<T>
where
  T: Value,
{
  fn from(contents: Option<T>) -> Self {
    Self { contents }
  }
}

impl<T> Value for OptionalValue<T> where T: Value {}

type Key = Setting;

// struct Schema {
//   settings_map: HashMap<Key, &Definition>,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserSettings {
  values: HashMap<Key, Definition<dyn Value>>,
}

fn settings_definition() -> Vec<Definition<dyn Value>> {
  let mut settings = Vec::new();
  settings.push(Definition {
    key: Setting::AppDataDirectory,
    name: "App Data Directory",
    description: "The directory where Minecraft Save Manager stores all of its data.",
    optional: false,
    data_type: DataType::String,
    input_type: InputType::PathFolder,
    default_value: Box::new(StringValue::from(String::from("."))),
  });
  settings.push(Definition {
    key: Setting::ServerDirectory,
    name: "Server Directory",
    description: r"\
    The working directory of the Minecraft server.\
    If left empty, the directory will be created in a temporary location on server startup and deleted on server shutdown.\
    Otherwise, select a custom directory in which to keep the last world that was run on the server.",
    optional: true,
    data_type: DataType::String,
    input_type: InputType::PathFolder,
    default_value: Box::new(OptionalValue::<StringValue>::from(None)),
  });

  settings
}

impl UserSettings {
  // fn get_schema() -> Schema {
  //   let settings_map = HashMap::new();

  //   for (index, &definition) in SETTINGS.iter().enumerate() {
  //     settings_map.insert(index, definition);
  //   }

  //   Schema { settings_map }
  // }
}

fn special_clone(value: &DynamicValue) -> DynamicValue {
  Box::new(value.clone())
}

impl Default for UserSettings {
  fn default() -> Self {
    Self {
      values: settings_definition().iter().map(|setting| (setting.key, special_clone(&setting.default_value))).collect(),
    }
  }
}
