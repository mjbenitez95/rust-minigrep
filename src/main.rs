use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!(
        "This program will search for \"{}\" in file {}",
        query, filename
    );

    let contents =
        fs::read_to_string(filename).expect("Something went wrong with reading the file");

    println!("The file {} has contents: \n\n{}", filename, contents);
}
