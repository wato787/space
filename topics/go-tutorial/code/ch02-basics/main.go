package main

import "fmt"

const Pi = 3.14

func add(x, y int) int {
	return x + y
}

func swap(a, b string) (string, string) {
	return b, a
}

func main() {
	var i int = 40
	j := 2
	sum := add(i, j)

	a, b := swap("go", "lang")

	fmt.Println("sum:", sum)
	fmt.Println("swap:", a, b)
	fmt.Println("pi:", Pi)
}

