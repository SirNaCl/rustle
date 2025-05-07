use once_cell::sync::Lazy;
use std::collections::HashSet;

const FILE_CONTENTS: &str = include_str!("wordlist.txt");
static WORDLIST: Lazy<HashSet<String>> =
    Lazy::new(|| FILE_CONTENTS.lines().map(String::from).collect());

pub fn wordlist_get_rnd() -> String {
    WORDLIST.iter().next().unwrap().to_owned()
}

pub fn wordlist_contains(value: &String) -> bool {
    WORDLIST.contains(value)
}
