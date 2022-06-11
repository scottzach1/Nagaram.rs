use std::{env, fs};
use std::collections::HashMap;
use std::str::{Lines};

#[derive(Debug)]
struct Node {
    path: String,
    root: bool,
    children: HashMap<char, Node>,
}

impl Node {
    fn default() -> Node {
        Node {
            path: format!(""),
            root: false,
            children: HashMap::new(),
        }
    }
    fn from_str(str: String) -> Node { Node { path: str, ..Node::default() } }

    fn ingest(self, word: String) -> Node {
        if word.is_empty() { return self; }
        let (c, rest) = get_c(&word);

        let mut children = self.children;
        let child = children
            .remove(&c)
            .unwrap_or(Node::from_str(format!("{}{}", &self.path, c)))
            .ingest(rest);
        children.insert(c, child);

        Node {
            path: self.path,
            children,
            ..Node::default()
        }
    }
}

fn parse(iterator: Lines) -> Node {
    iterator.fold(Node::default(), |n, line| n.ingest(line.to_string()))
}

/// Extract the next character from a String, returning bot the next char and remainder.
fn get_c(s: &String) -> (char, String) {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => (c, chars.collect()),
        None => panic!("unexpected end of string"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let dictionary = fs::read_to_string("words.txt")
        .expect("Could not locate dictionary");

    parse(dictionary.lines());
}
