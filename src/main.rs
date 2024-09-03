mod image;
pub use crate::image::rename_image;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Hoarder")
        .version("1.0")
        .author("Evan Alvarez")
        .about("The greatest renaming utility the world has ever known")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .help("Path to the thing(s) you want to be renamed")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .help("Organize files into a directory based structure")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let path: Vec<&String> = matches.get_many("path").unwrap().collect();
    if matches.get_flag("directory") {
        rename_image::directory(path);
    } else {
        rename_image::single(path);
    };
}
