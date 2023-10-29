package main

import (
	"fmt"
	"runtime/debug"
)

func main() {
	// Defer a function that recovers from a panic.
	defer func() {
		if r := recover(); r != nil {
			fmt.Println(string(debug.Stack()))
		}
	}()

	// Simulate a panic by dividing by zero.
	a := 10
	b := 0
	result := a / b // This will panic.

	// This line will not be reached due to the panic.
	fmt.Println("Result:", result)
}
