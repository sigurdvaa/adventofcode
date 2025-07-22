package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	fmt.Println("Hello, World!")
	argsWithProg := os.Args
	argsWithoutProg := os.Args[1:]
	lastArg := os.Args[len(os.Args)-1]

	fmt.Println("argsWithProg: ", argsWithProg)
	fmt.Println("argsWithoutProg: ", argsWithoutProg)
	fmt.Println("lastArg: ", lastArg)

	var num int
	flag.IntVar(&num, "d", 0, "Specify which day of the Advent of Code to run. Default is 0 (all)")
	flag.Usage = func() {
		fmt.Printf("Usage: %s [-d <0-25>]", os.Args[0])
	}
	flag.Parse()
	fmt.Println("Day: ", num)
	if num > 25 {
		fmt.Printf("Day is out of range (0-25): %d\n", num)
		os.Exit(1)
	}
}
