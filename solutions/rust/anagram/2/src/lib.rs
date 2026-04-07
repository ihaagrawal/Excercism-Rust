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
    let w = word.to_lowercase();
    let st = s.to_lowercase();

    if w == st {
        return false;
    }

    let mut p: Vec<char> = w.chars().collect();
    let mut t: Vec<char> = st.chars().collect();

    p.sort_unstable();
    t.sort_unstable();

    p == t
}