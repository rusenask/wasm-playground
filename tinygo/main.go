package main

import "fmt"

// This calls a JS function from Go.
func main() {}

// This function is imported from JavaScript, as it doesn't define a body.
// You should define a function named 'main.add' in the WebAssembly 'env'
// module from JavaScript.
// func add(x, y int)

// This function is exported to JavaScript, so can be called using
// exports.handleRequest() in JavaScript.
//go:export handleRequest
func handleRequest(request string) int {
	fmt.Println(request)
	return 5
}
