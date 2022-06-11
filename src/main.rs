use std::{env, fs};
use std::iter::Map;

#[derive(Debug)]
struct Node {
    path: String,
    parent: Option<Box<Node>>,
    children: Option<Map<String, Box<Node>>>,
}

impl Node {
    fn get_root(self) -> Node {
        match self.parent {
            Some(parent) => parent.get_root(),
            None => self
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let dictionary = fs::read_to_string("words.txt")
        .expect("Could not locate dictionary");

    println!("Dictionary:");
    dictionary.lines().into_iter().for_each(|word|println!("- {}", word));
}
