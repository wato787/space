package main

import "fmt"

type Counter struct {
	value int
}

func (c *Counter) Inc() {
	c.value++
}

func (c Counter) Value() int {
	return c.value
}

type Greeter interface {
	Greet() string
}

type Person struct {
	Name string
}

func (p Person) Greet() string {
	return "Hello, " + p.Name
}

func say(g Greeter) {
	fmt.Println(g.Greet())
}

func main() {
	c := &Counter{}
	c.Inc()
	c.Inc()
	fmt.Println("counter:", c.Value())

	say(Person{Name: "Go"})
}

