use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = args.len() > 3 && args[3].clone() == "--ignore-case"
            || (env::var("IGNORE_CASE")
                .unwrap_or(String::from("false"))
                .eq("true"));
        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut result = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            let mut result_line = "".to_owned();
            for word in line.split(" ") {
                let mut result_word = word.to_owned();
                if result_word.contains(query) {
                    result_word = format!("{}{}{}",
                        "\x1b[31m",
                        word,
                        "\x1b[0m");
                } 
                if result_line.len() > 0 {
                    result_line.push_str(" ");
                }
                result_line.push_str(&result_word);
            }
            result.push(result_line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut result = vec![];
    let insesitive_query = query.to_lowercase();
    for line in contents.lines() {
        if line.trim().to_lowercase().contains(&insesitive_query) {
            let mut result_line = "".to_owned();
            for word in line.split(" ") {
                let mut result_word = word.to_lowercase().to_owned();
                if result_word.contains(&insesitive_query) {
                    result_word = format!("{}{}{}",
                        "\x1b[31m",
                        word,
                        "\x1b[0m");
                } 
                if result_line.len() > 0 {
                    result_line.push_str(" ");
                }
                result_line.push_str(&result_word);
            }
            result.push(result_line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, \u{1b}[31mproductive.\u{1b}[0m".to_owned()],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["\u{1b}[31mRust:\u{1b}[0m".to_owned(), "\u{1b}[31mTrust\u{1b}[0m me.".to_owned()],
            search_case_insensitive(query, contents)
        );
    }
}
