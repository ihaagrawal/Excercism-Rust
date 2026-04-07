pub fn reverse(input: &str) -> String {
    if input.is_empty(){
        return String::new();
    }
    let mut chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut j = chars.len().saturating_sub(1);
    while i <= j{
        chars.swap(i, j);
        i += 1; j -= 1;
    }
    chars.into_iter().collect()
}
