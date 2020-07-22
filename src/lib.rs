mod config;
pub use config::config::Config;

use std::fs;

pub fn run(config: &Config) {
    let contents = fs::read_to_string(&config.path_file)
        .expect("Something wrong with reading from the file.");

    println!("Searching for: {}", config.query);

    let result;

    if config.case_insensitive {
        result = search_case_insensitive(&config.query, &contents)
    } else {
        result = search(&config.query, &contents);
    }

    for line in result {
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
pub fn search_case_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
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
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "DuCT";
        let contents = "\
Rust:
safe, fast, ProduCtivE.
Pick three.";
        assert_eq!(vec!["safe, fast, ProduCtivE."], search_case_insensitive(query, contents));
    }
}