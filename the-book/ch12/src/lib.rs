// Taken from Chapter 12 of The Rust Programming Language
use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Please specify both `query` and `file_path` args")
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            Ok(Config { query, file_path})
        }
    }
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut hits: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            hits.push(line)
        }
    }
    hits
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;
    let results = search(&config.query, &text);
    println!("Results:");
    for result in results {
        println!("  {result}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let arg1 = String::from("foo");
        let arg2 = String::from("bar.txt");
        let args = vec![String::from(""), arg1.clone(), arg2.clone()];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, arg1);
        assert_eq!(config.file_path, arg2);
    }

    #[test]
    fn test_search_true_positives() {
        let query = String::from("Foo");
        let contents = String::from("Foo\nBar\nLa la la\nFoobar");
        assert_eq!(vec!["Foo", "Foobar"], search(&query, &contents));
    }

    #[test]
    fn test_search_false_positives() {
        let query = String::from("Baz");
        let contents = String::from("Foo\nBar\nLa la la\nFoobar");
        assert_eq!(search(&query, &contents).len(), 0);
    }
}
