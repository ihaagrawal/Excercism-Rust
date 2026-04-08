pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    let len = num.to_string().len();
    for c in num.to_string().chars(){
        let n = c.to_digit(10).unwrap();
        sum += n.pow(len as u32);
    }
    sum == num
}
