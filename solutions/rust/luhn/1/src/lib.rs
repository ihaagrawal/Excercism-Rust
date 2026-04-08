/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let s = code.replace(" ", "");

    if s.len() <= 1 || !s.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let mut digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut i = digits.len() as i32 - 2;

    while i >= 0{
        let idx = i as usize;
        digits[idx] *= 2;

        if digits[idx] > 9 {
            digits[idx] -= 9;
        }

        i -= 2;
    }

    let mut j = 0;
    let mut sum = 0;
    while j < digits.len(){
        sum += digits[j];
        j += 1;
    }
    
    sum % 10 == 0
}
