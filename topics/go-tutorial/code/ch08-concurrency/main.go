package main

import (
	"fmt"
	"time"
)

func worker(id int, ch chan<- string) {
	time.Sleep(time.Duration(id) * 100 * time.Millisecond)
	ch <- fmt.Sprintf("worker %d done", id)
}

func main() {
	ch := make(chan string)

	go worker(1, ch)
	go worker(2, ch)

	for i := 0; i < 2; i++ {
		fmt.Println(<-ch)
	}
}

