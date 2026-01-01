use std::env;
use std::fs;

fn main() {

    // Collect arfuments from command line
    let args: Vec<String> = env::args().collect();

    // Store arg for query and file_path
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // Get content from file in file path
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
