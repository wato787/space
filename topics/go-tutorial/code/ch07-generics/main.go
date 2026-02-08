package main

import "fmt"

type Number interface {
	~int | ~int64 | ~float64
}

func Sum[T Number](values []T) T {
	var total T
	for _, v := range values {
		total += v
	}
	return total
}

func Map[T any, R any](in []T, fn func(T) R) []R {
	out := make([]R, 0, len(in))
	for _, v := range in {
		out = append(out, fn(v))
	}
	return out
}

func main() {
	nums := []int{1, 2, 3}
	fmt.Println("sum:", Sum(nums))

	labels := Map(nums, func(n int) string {
		return fmt.Sprintf("n=%d", n)
	})
	fmt.Println("labels:", labels)
}

