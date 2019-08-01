extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "f")]
struct Opt {
    #[structopt(long = "add")]
    add: Option<String>,

    #[structopt(name = "PATH", parse(from_os_str))]
    path: Option<PathBuf>,

}

fn main() {
    let opt = Opt::from_args();
    match (opt.add, opt.path) {
        (None, None) => println!("Nothing"),
        (Some(add), Some(path)) => println!("{:#?} {:#?}", add, path),
        (Some(add), None) => println!("{:#?}", add),
        (None, Some(path)) => println!("{:#?}", path),
    }
}
