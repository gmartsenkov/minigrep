use std::env;
use std::process;
use minigrep::Config;

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

    minigrep::run(&config);
}
