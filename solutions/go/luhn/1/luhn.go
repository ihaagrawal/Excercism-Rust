package luhn

func Valid(id string) bool {
    var sum int
    digits := 0
    isEven := false
    
    // Process from right to left
    for i := len(id) - 1; i >= 0; i-- {
        if id[i] == ' ' {
            continue
        }
        if id[i] < '0' || id[i] > '9' {
            return false
        }
        
        digit := int(id[i] - '0')
        digits++
        
        if isEven {
            digit *= 2
            if digit > 9 {
                digit -= 9
            }
        }
        
        sum += digit
        isEven = !isEven
    }
    
    return digits > 1 && sum%10 == 0
}   