// Fig. 1. Example for local value numbering

package main

func main() {
    a := 42
    b := a
    c := a + b

    a = c + 23
    c = a + b
}