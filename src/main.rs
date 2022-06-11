extern crate core;

use std::{env, fs};
use std::collections::{HashMap, HashSet};
use std::str::{Lines};

struct Dictionary {
    path: String,
    word: bool,
    children: HashMap<char, Dictionary>,
}

impl Dictionary {
    /// Provide a default value to build from
    fn default() -> Dictionary {
        Dictionary {
            path: format!(""),
            word: false,
            children: HashMap::new(),
        }
    }
    //// Construct a default node from a provided string
    fn from_str(str: String) -> Dictionary { Dictionary { path: str, ..Dictionary::default() } }

    /// Add a word into the dictionary
    fn add(self, word: String) -> Dictionary {
        if word.is_empty() { return Dictionary { word: true, ..self }; }
        let (c, rest) = get_c(&word);

        let mut children = self.children;
        let child = children
            .remove(&c)
            .unwrap_or(Dictionary::from_str(format!("{}{}", &self.path, c)))
            .add(rest);
        children.insert(c, child);

        Dictionary { children, ..self }
    }

    /// Helper method to find anagrams with root reference
    fn anagrams_helper(&self, word: String, root: &Dictionary) -> Vec<String> {
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

    /// Get all anagrams that match a provided word
    fn anagrams(&self, word: String) -> Vec<String> {
        self.anagrams_helper(word, &self)
    }
}

/// Parse an iterator of words into a Dictionary
fn parse(iterator: Lines) -> Dictionary {
    iterator.fold(Dictionary::default(), |n, line| n.add(line.to_string().to_lowercase()))
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

/// Remove any whitespace from a string
fn remove_whitespace(string: &str) -> String {
    string.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Prints a list of anagrams for a provided word
fn get_anagrams(word: String) {
    let target = remove_whitespace(&word).to_lowercase();
    let words = fs::read_to_string("words.txt").expect("Could not locate dictionary");
    let root = parse(words.lines());

    let anagrams = root.anagrams(target);
    anagrams.iter().for_each(|w| println!("- {:#?}", w));
}

/// Program entrypoint - print anagrams for word in argv
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
