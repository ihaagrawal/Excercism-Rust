package isbnverifier

func IsValidISBN(isbn string) bool {
    // Remove dashes
    var cleaned []byte
    for i := 0; i < len(isbn); i++ {
        if isbn[i] != '-' {
            cleaned = append(cleaned, isbn[i])
        }
    }
    
    // Must be exactly 10 characters
    if len(cleaned) != 10 {
        return false
    }
    
    sum := 0
    for i, char := range cleaned {
        weight := 10 - i
        
        var digit int
        if i == 9 && (char == 'X' || char == 'x') {
            digit = 10
        } else if char >= '0' && char <= '9' {
            digit = int(char - '0')
        } else {
            return false // Invalid character
        }
        
        sum += digit * weight
    }
    
    return sum%11 == 0
}   