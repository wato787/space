package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func main() {
	arr := [3]int{1, 2, 3}
	slice := []int{1, 2, 3}
	slice = append(slice, 4)

	scores := map[string]int{
		"alice": 90,
		"bob":   82,
	}

	p := Person{Name: "Chris", Age: 28}

	fmt.Println("array:", arr)
	fmt.Println("slice:", slice)
	fmt.Println("map:", scores)
	fmt.Println("person:", p.Name, p.Age)
}

