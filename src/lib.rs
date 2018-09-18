use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        // if "CASE_INSENSITIVE" env var is set to anything return false
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        args.next(); // skip first arg (unneeded program name)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("You must provide at least two arguments!")
        };

        let filename = match args.next() {
            Some(fname) => fname,
            None => return Err("You must provide at least two arguments!")
        };

        // This takes an optional third command line arg which will set the search
        // to case insensitive, should probably extract remaining args into an array
        // here to provide list of user options to apply to the search
        let case_sensitive_arg = match args.next() {
            Some(value) => case_sensitive = false,
            None => ()
        };


        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    match config.case_sensitive {
        true => {
            for line in search(&config.query, &contents) {
                println!("{}\n\r", line);
            }
        },
        false => {
            for line in search_case_insensitive(&config.query, &contents) {
                println!("{}\n\r", line);
            }
        }
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| { line.contains(query) })
        .collect()
}
/*
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
*/

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| {
            line.to_lowercase().contains(&query)
        })
        .collect()
}



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

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        //let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

        // multiline literal string
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust Me.";

        assert_eq!(
            vec!["Rust:", "Trust Me."],
            search_case_insensitive(query, contents)
        )
    }
}
