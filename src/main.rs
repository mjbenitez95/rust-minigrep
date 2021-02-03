use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!(
        "This program will search for \"{}\" in file {}",
        config.query, config.filename
    );

    let contents =
        fs::read_to_string(&config.filename).expect("Something went wrong with reading the file");

    println!("The file {} has contents: \n\n{}", &config.filename, contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Config { query, filename }
    }
}
