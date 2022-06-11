use std::{env, fs};
use std::iter::Map;
use std::str::Lines;

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

fn parse(iterator: Lines) {
    println!("Dictionary:");
    iterator.for_each(|word|println!("- {}", word));
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let dictionary = fs::read_to_string("words.txt")
        .expect("Could not locate dictionary");

    parse(dictionary.lines());
}
