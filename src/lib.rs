use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file)?;

    println!("{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let file: String = args[2].clone();
        let query: String = args[1].clone();
        Ok(Config { query, file })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let content: &str = "/
Rust:
safe, fast, productive.
Pick three. ";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
