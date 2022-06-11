use std::{env, fs};
use std::iter::Map;

#[derive(Debug)]
struct Node {
    path: String,
    end: bool,
    parent: Box<Node>,
    children: Map<String, Box<Node>>,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let dictionary = fs::read_to_string("words.txt")
        .expect("Could not locate dictionary");

    println!("Dictionary:");
    dictionary.lines().into_iter().for_each(|word|println!("- {}", word));
}
