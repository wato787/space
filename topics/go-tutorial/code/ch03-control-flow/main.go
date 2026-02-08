package main

import "fmt"

func main() {
	for i := 0; i < 3; i++ {
		fmt.Println("loop:", i)
	}

	if n := 5; n%2 == 1 {
		fmt.Println("odd:", n)
	}

	day := "sun"
	switch day {
	case "sat", "sun":
		fmt.Println("weekend")
	default:
		fmt.Println("weekday")
	}

	defer fmt.Println("defer: done")
	fmt.Println("end")
}

