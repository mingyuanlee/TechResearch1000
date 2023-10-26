package main

import "fmt"

func main() {
	b := []int{1, 2, 3, 4, 5}
	a := b[:] // Create a new slice a that references the same array as b

	a[0] = 100

	fmt.Println("b:", b)
	fmt.Println("a:", a)
}
