use std::env;
use std::fs;
use std::process;
use minigrep::config::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config;
    match Config::new(args.as_slice()) {
        Ok(conf) => config = conf,
        Err(err) => {
            println!("An error occurred: {}", err);
            process::exit(1);
        }
    }

    let contents = fs::read_to_string(&config.path_file)
                            .expect("Something wrong with reading from the file.");

    println!("Searching for {:?}", config);
    println!("Contents of the file: \n ---BEGIN--- \n {} \n ---END---", contents);
}
