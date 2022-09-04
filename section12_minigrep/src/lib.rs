use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() > 3 {
            return Err("Too many arguments specified!");
        }
        let query_result = args.get(1);
        let file_path_result = args.get(2);
        let query = match query_result {
            Some(q) => q.clone(),
            None => return Err("No query argument specified!"),
        };
        let file_path = match file_path_result {
            Some(f) => f.clone(),
            None => return Err("No file_path argument specified!"),
        };
        Ok(Config { query, file_path })
    }
}

pub fn run<'a>(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for l in contents.lines().map(|x| x.trim()) {
        if l.contains(query) {
            result.push(l);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn one_frog() {
        let config = Config::build(&[
            String::from(""),
            String::from("frog"),
            String::from("poem.txt"),
        ])
        .unwrap();
        let contents = fs::read_to_string(config.file_path).unwrap();
        assert_eq!(search(&config.query, &contents).len(), 1);
    }
}
