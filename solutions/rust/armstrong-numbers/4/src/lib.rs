pub fn is_armstrong_number(num: u32) -> bool {
    let mut temp = num;
    let mut len = 0;

    while temp > 0{
        len += 1;
        temp /= 10;
    }

    if len == 0{
        len = 1;
    }

    let mut temp = num;
    let mut sum = 0;

    while temp > 0{
        sum += (temp % 10).pow(len);
        temp /= 10;
    }

    sum == num
}