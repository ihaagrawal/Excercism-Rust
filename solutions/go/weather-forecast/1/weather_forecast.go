// Package weather provides current weather information for a location.
package weather

// CurrentCondition represents the current weather condition.
var CurrentCondition string

// CurrentLocation represents the current city or location.
var CurrentLocation string

// Forecast sets the current location and condition and returns a formatted weather forecast string.
func Forecast(city, condition string) string {
    CurrentLocation, CurrentCondition = city, condition
    return CurrentLocation + " - current weather condition: " + CurrentCondition
}   