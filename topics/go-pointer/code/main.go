package main

import "fmt"

type Counter struct {
	value int
}

func (c *Counter) Inc() {
	c.value++
}

func increment(n *int) {
	*n = *n + 1
}

func swap(a, b *int) {
	*a, *b = *b, *a
}

func main() {
	value := 10
	increment(&value)
	fmt.Printf("value=%d\n", value)

	left, right := 1, 2
	swap(&left, &right)
	fmt.Printf("left=%d right=%d\n", left, right)

	counter := Counter{value: 3}
	counter.Inc()
	fmt.Printf("counter=%d\n", counter.value)

	counterPtr := new(Counter)
	counterPtr.Inc()
	fmt.Printf("counterPtr=%d\n", counterPtr.value)

	zero := new(int)
	fmt.Printf("zero=%d\n", *zero)

	var maybe *int
	fmt.Printf("maybe nil? %v\n", maybe == nil)
	if maybe == nil {
		maybe = &value
	}
	fmt.Printf("maybe points to %d\n", *maybe)
}
