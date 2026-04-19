package cars

func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	return (float64(productionRate) * successRate) / float64(100)
}

func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	var successfullCarsPerHour float64 = (float64(productionRate) * successRate) / float64(100)
    return int(successfullCarsPerHour) / 60
}

func CalculateCost(carsCount int) uint {
	groupsOfTen := carsCount / 10
    remainingCars := carsCount % 10
    cost := groupsOfTen * 95000 + remainingCars * 10000
	return uint(cost)
}
