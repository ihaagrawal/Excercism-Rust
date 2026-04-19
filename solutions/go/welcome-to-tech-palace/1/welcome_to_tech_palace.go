package techpalace
import (
    "strings"
    "unicode"
)

func WelcomeMessage(customer string) string {
	return "Welcome to the Tech Palace, " + strings.ToUpper(customer)
}

func AddBorder(welcomeMsg string, numStarsPerLine int) string {
	border := strings.Repeat("*", numStarsPerLine)
	return border + "\n" + welcomeMsg + "\n" + border
}

func CleanupMessage(oldMsg string) string {
    // Keep only letters, digits, and spaces
    cleaned := strings.Map(func(r rune) rune {
        if unicode.IsLetter(r) || unicode.IsDigit(r) || r == ' ' || r == '%' || r == ','{
            return unicode.ToUpper(r)
        }
        return -1
    }, oldMsg)
    
    // Trim surrounding whitespace and collapse multiple spaces
    cleaned = strings.TrimSpace(cleaned)
    return cleaned
}   