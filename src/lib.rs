mod config;
pub use config::config::Config;

use std::fs;

pub fn run(config: &Config) {
    let contents = fs::read_to_string(&config.path_file)
        .expect("Something wrong with reading from the file.");

    println!("Searching for: {}", config.query);

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
}

pub fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}