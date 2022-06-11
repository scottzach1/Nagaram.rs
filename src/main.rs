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

    fn ingest(self, word: String) -> Node {
        if word.is_empty() { return self }

        let (c, rest) = get_c(word);
        let child_default = Node { path: format!("{}{}", &self.path, c), ..Node::default() };

        let mut children = self.children;
        let child = children
            .remove(&c).unwrap_or(child_default)
            .ingest(rest);
        children.insert(c, child);

        Node {
            path: self.path,
            children,
            ..Node::default()
        }
    }
}

fn parse(_iterator: Lines) -> Node {
    println!("Dictionary:");
    // iterator.for_each(|word|println!("- {}", word));

    let root = Node::default().ingest(format!("abc")).ingest(format!("ad"));
    println!("{:#?}", root);
    root
}

/// Extract the next character from a String, returning bot the next char and remainder.
/// Throws a `SyntaxError` on end of string.
fn get_c(s: String) -> (char, String) {
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
