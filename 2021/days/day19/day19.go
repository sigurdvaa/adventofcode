package day19

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

type Coord struct {
	x int
	y int
	z int
}

func parseInput(str string) [][]Coord {
	coords := [][]Coord{}

	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
	}

	return coords
}

func Run() {
	fmt.Println("Day 19: Beacon Scanner")

	inputString := input.ReadDay("day19")
	_ = parseInput(inputString)

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
