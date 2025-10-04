package main
import (
	"fmt"
)

// Constant declaration
const f = 5.0

func main() {
	// Explicit declaration and initialization of a variable
	var x int = 10
	// "Short" variable declaration and initialization
	y := 20
	x = 5
	var z = x + y + f

	fmt.Println("The number is:", z)
}