use std::{env::args, process};

use file_flier::config::Config;

fn main() {
    let args = args().skip(1);

    let Ok(config) = Config::new(args) else {
        println!("Couldn't create config from given arguments");
        process::exit(1);
    };

    if let Err(err) = file_flier::run(config) {
        println!("Error occured while running program\n{err}");
        process::exit(1);
    }

    // println!("{config:?}");
}
