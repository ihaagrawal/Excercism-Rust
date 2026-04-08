pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), c| {
            let mut n = c.to_digit(10)?;

            if count % 2 == 1 {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }

            Some((sum + n, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}