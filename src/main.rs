mod add;
mod search;
mod store;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "f")]
struct Opt {
    #[structopt(long = "add")]
    add: Option<PathBuf>,

    #[structopt(name = "PATH", parse(from_os_str))]
    path: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    match (opt.add, opt.path) {
        (None, None) => println!("Nothing"),
        (Some(_), Some(_)) => println!("Too many args"),
        (Some(add_path), None) => add::add(add_path),
        (None, Some(path)) => search::search(path),
    }
}
