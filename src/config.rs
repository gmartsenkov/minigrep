pub mod config {
    #[derive(Debug)]
    pub struct Config {
        pub query : String,
        pub path_file: String,
        pub case_insensitive: bool
    }

    impl Config {
        pub fn new(args : &[String]) -> Result<Config, &'static str> {

            if args.len() < 3 {
                return Err("not enough arguments")
            }

            let query = args[1].clone();
            let path_file = args[2].clone();

            let case_insensitive = std::env::var("CASE_INSENSITIVE").is_err();

            Ok(Config { query, path_file, case_insensitive })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_with_no_args() {
            let args: &[String] = &[];
            assert!(Config::new(args).is_err());
        }

        #[test]
        fn new_with_one_args() {
            let args : [String; 1] = ["bob".to_string()];
            assert!(Config::new(&args).is_err());
        }

        #[test]
        fn new_with_two_args() {
            let args : [String; 2] = ["bob".to_string(), "nike".to_string()];
            assert!(Config::new(&args).is_err());
        }

        #[test]
        fn new_with_three_args() {
            let args : [String; 3] = ["bob".to_string(), "nike".to_string(), "mark".to_string()];
            assert!(Config::new(&args).is_ok());
        }
    }
}