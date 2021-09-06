use std::error::Error;
use std::{fs, env};

pub struct Config {
    needle: String,
    f_name: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 { return Err("Not enough arguments!"); }
        else if args.len() > 3 { return Err("Too many arguments!"); }

        let needle = args[1].clone();
        let f_name = args[2].clone();

        // It's case sensitive if checking the env-var returns an Err; is_err
        let case_sensitive = env::var("CI").is_err();

        Ok(Config { needle, f_name, case_sensitive })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let haystack = fs::read_to_string(config.f_name)?;

    let lines = if config.case_sensitive {
        search(&config.needle, &haystack)
    } else {
        search_case_insensitive(&config.needle, &haystack)
    };

    let width = lines.len().to_string().len();

    for (i, line) in lines.iter().enumerate() {
        println!("{:>w$}: {}", i + 1, line, w = width);
    }

    Ok(())
}


fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }

    results
}


fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let needle = needle.to_lowercase();
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.to_lowercase().contains(&needle) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let needle = "duct";
        let haystack = "\
Rust:
safe, fast, productive;
pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive;"],
            search(needle, haystack)
        )
    }

    #[test]
    fn case_insensitive() {
        let needle = "rUSt";
        let haystack = "\
Rust:
safe, fast, productive;
pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(needle, haystack)
        )
    }

}