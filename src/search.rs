use crate::store;
use std::path::PathBuf;
use sublime_fuzzy::best_match;

#[derive(Debug)]
struct Match {
  pathname: String,
  score: isize,
}

pub fn search(input: PathBuf) {
  let string = input.to_str().expect("oh god o h no");

  let items: Vec<store::Item> = match store::read_config() {
    Some(items) => items,
    None => Vec::new(),
  };

  let iter = items.clone().into_iter();

  let mut matches = Vec::new();
  for item in iter {
    match best_match(&string, &item.pathname) {
      Some(m) => matches.push(Match {
        pathname: item.pathname,
        score: m.score(),
      }),
      None => (),
    }
  }

  matches.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

  if !matches.is_empty() {
    let found = matches.first().unwrap();
    println!("{}", found.pathname);
  }
}
