extern crate core;

use std::{env, fs};
use std::collections::{HashMap, HashSet};
use std::str::{Lines};

struct Node {
    path: String,
    word: bool,
    children: HashMap<char, Node>,
}

impl Node {
    fn default() -> Node {
        Node {
            path: format!(""),
            word: false,
            children: HashMap::new(),
        }
    }
    fn from_str(str: String) -> Node { Node { path: str, ..Node::default() } }

    fn ingest(self, word: String) -> Node {
        if word.is_empty() { return Node { word: true, ..self }; }
        let (c, rest) = get_c(&word);

        let mut children = self.children;
        let child = children
            .remove(&c)
            .unwrap_or(Node::from_str(format!("{}{}", &self.path, c)))
            .ingest(rest);
        children.insert(c, child);

        Node {
            path: self.path,
            word: self.word,
            children,
            ..Node::default()
        }
    }

    fn anagrams_helper(&self, word: String, root: &Node) -> Vec<String> {
        let mut results = vec![];

        if word.is_empty() {
            if self.word {
                // Base case 1: Perfect match
                results.push(self.path.clone());
            }
            return results;
        }

        if self.word {
            // Base case 2: Hit end of path but still more string
            let remainder_results = root.anagrams_helper(word.clone(), root);
            results.extend(remainder_results.into_iter().map(|w| format!("{} {}", self.path, w)));
        }

        // Keep recursing deeper
        let char_set: HashSet<char> = HashSet::from_iter(word.chars());
        char_set.into_iter().fold(results, |mut results, c| {
            let rest = remove_c(&word, c).unwrap();
            match self.children.get(&c) {
                Some(child) => { results.extend(child.anagrams_helper(rest, root)) }
                None => {}
            };
            results
        })
    }

    fn anagrams(&self, word: String) -> Vec<String> {
        self.anagrams_helper(word, &self)
    }
}

fn parse(iterator: Lines) -> Node {
    iterator.fold(Node::default(), |n, line| n.ingest(line.to_string().to_lowercase()))
}

/// Remove the first instance of a character from a string
fn remove_c(subject: &String, char: char) -> Option<String> {
    let index = subject.find(char);
    match index {
        Some(i) => {
            let mut subject = subject.clone();
            subject.remove(i);
            Some(subject)
        }
        None => None
    }
}

/// Extract the next character from a String, returning bot the next char and remainder.
fn get_c(s: &String) -> (char, String) {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => (c, chars.collect()),
        None => panic!("unexpected end of string"),
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_anagrams(target: String) {
    let target = remove_whitespace(&target).to_lowercase();
    let words = fs::read_to_string("words.txt").expect("Could not locate dictionary");
    let root = parse(words.lines());

    let anagrams = root.anagrams(target);
    anagrams.iter().for_each(|w| println!("- {:#?}", w));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let target = match args.len() {
        0..=1 => {
            println!("Missing second word argument!");
            None
        }
        2 => Some(&args[1]),
        n => {
            println!("Unexpected arguments! {:#?}", &args[2..n]);
            Some(&args[1])
        }
    };

    match target {
        Some(target) => { get_anagrams(target.clone()) }
        None => {}
    }
}
