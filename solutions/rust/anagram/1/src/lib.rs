use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&str> = HashSet::new();

    for &s in possible_anagrams{
        if check(s, word){
            set.insert(s);
        }
    }

    set
}

pub fn check(word: &str, s: &str) -> bool {
    if word.to_lowercase() == s.to_lowercase() {
        return false;
    }

    if word.len() != s.len() {
        return false;
    }

    let mut p: Vec<char> = word.to_lowercase().chars().collect();
    let mut t: Vec<char> = s.to_lowercase().chars().collect();

    p.sort_unstable();
    t.sort_unstable();

    p == t
}