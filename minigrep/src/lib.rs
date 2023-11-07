use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        //the first arg is the name of the program, skip that
        args.next();

        let query = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a query string")
        };

        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a file path string")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for mut line in contents.lines() {
        line = line.trim();
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "find";
        let contents = "\
        This is a test string, 
        you should find
        this.";

        assert_eq!(vec!["you should find"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "lEtHaRGY";
        let contents = "\
        This is a test string, 
        lethargy,
        you should find
        this.";

        assert_eq!(
            vec!["lethargy,"],
            search_case_sensitive(query, contents)
        );
    }
}