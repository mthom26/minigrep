extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    /*
    // Older code before Config was updated
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let config = match config {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Problem passing arguments: {}", e);
            process::exit(1);
        }
    };
    */

    /*
    // Cleaner version of initializing config from rust book
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });
    */

    println!("Searching for '{}' in {}\n\r", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

