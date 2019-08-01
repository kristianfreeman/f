use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
  pub pathname: String,
  // pub weight: u32,
  // #[serde(skip_serializing_if = "Option::is_none")]
  // pub alias: Option<String>,
}

pub fn config_file() -> String {
  let home_directory = "/Users/kristian";
  format!("{}/.f.json", home_directory)
}

pub fn read_config() -> Option<Vec<Item>> {
  let contents = fs::read_to_string(config_file()).expect("Something went wrong reading the file");

  if contents.is_empty() {
    None
  } else {
    let items: Vec<Item> = serde_json::from_str(&contents).unwrap();
    Some(items)
  }
}

pub fn write_item(path: PathBuf) -> Result<(), failure::Error> {
  let mut items = Vec::new();

  match read_config() {
    Some(read) => items.extend(read),
    None => (),
  };

  let pathname = path.to_str().expect("oh god oh no");
  let mut items_iter = items.clone().into_iter();
  let existing = items_iter.find(|item| item.pathname == pathname);

  if existing.is_none() {
    let new_item = Item {
      pathname: pathname.to_string(),
    };
    items.push(new_item);

    let serialized = serde_json::to_string(&items)?;
    fs::write(&config_file(), &serialized)?;

    println!("{:#?}", serialized);
  }

  Ok(())
}
