package main

import (
	"aoc_2021/days"
	"flag"
	"fmt"
	"os"
)

func main() {
	var num int
	flag.IntVar(&num, "d", 0, "Specify which day of the Advent of Code to run. Default is 0 (all).")
	flag.Usage = func() {
		fmt.Printf("Usage: %s [-d <0-25>]", os.Args[0])
	}
	flag.Parse()

	if num > 25 {
		fmt.Printf("Day is out of range (0-25): %d\n", num)
		os.Exit(1)
	}

	days.Run(num)
}
