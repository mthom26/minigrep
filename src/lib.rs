use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub print_line_number: bool,
    pub return_one_line: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        // if "CASE_INSENSITIVE" env var is set to anything return false
        // Disabled for now, need to figure out how to combine this with
        // checking for manual cl args below
        // let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        args.next(); // skip first arg (unneeded program name)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("You must provide at least two arguments!")
        };

        let filename = match args.next() {
            Some(fname) => fname,
            None => return Err("You must provide at least two arguments!")
        };

        let (
            mut case_sensitive,
            mut print_line_number,
            mut return_one_line) = (
            false, false, false
        );

        // Go through args to set config options
        while let Some(arg) = args.next() {
            println!("{}", &arg);
            match &arg[..] {
                "-c" => case_sensitive = true,
                "-p" => print_line_number = true,
                "-r" => return_one_line = true,
                _ => println!("NO ARGS!")
            }
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
            print_line_number,
            return_one_line
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let vec = search(&config.query, &contents, config.case_sensitive);
    println!("Found {} results for \"{}\"", vec.len(), &config.query);

    for(index, line) in vec {
        print_results(index, line, config.print_line_number);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<(usize, &'a str)> {
    match case_sensitive {
        true => {
            contents.lines().enumerate()
                .filter(|(index, line)| {
                    line.contains(query)
                })
                .collect()
        },
        false => {
            let query = query.to_lowercase();

            contents.lines().enumerate()
                .filter(|(index, line)| {
                    line.to_lowercase().contains(&query)
                })
                .collect()
        }
    }
}

fn print_results(index: usize, line: &str, print_line_number: bool) {
    match print_line_number {
        true => println!("{} {}", index, line),
        false => println!("{}", line)
    }
}

// TODO update tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        //let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

        // multiline literal string
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }
}
