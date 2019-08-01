use std::path::PathBuf;
use crate::store;

pub fn add(path: PathBuf) {
  store::write_item(path);
}