// +build js,wasm

package main

func main() {
	// fmt.Println("Hello World")
}

//go:export handleRequest
func handleRequest(request string) int {

	return 5
}
