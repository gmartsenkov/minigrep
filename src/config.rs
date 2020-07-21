pub mod config {
    #[derive(Debug)]
    pub struct Config {
        pub query : String,
        pub path_file: String
    }

    impl Config {
        pub fn new(args : &[String]) -> Result<Config, &'static str> {

            if args.len() < 3 {
                return Err("not enough arguments")
            }

            let query = args[1].clone();
            let path_file = args[2].clone();

            Ok(Config { query, path_file })
        }
    }
}