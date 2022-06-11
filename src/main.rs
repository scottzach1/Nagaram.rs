use std::{env, fs};


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let dictionary = fs::read_to_string("words.txt")
        .expect("Could not locate dictionary");

    println!("Dictionary:");
    dictionary.lines().into_iter().for_each(|word|println!("- {}", word));
}
