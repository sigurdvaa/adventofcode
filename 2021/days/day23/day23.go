package day23

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

const ROOMS int = 4
const SIZE int = 2

func parseInput(str string) [ROOMS][SIZE]rune {
	rooms := [ROOMS][SIZE]rune{}
	c := 0
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		for _, a := range line {
			if a >= 'A' && a <= 'D' {
				rooms[c%ROOMS][c/ROOMS] = a
				c += 1
			}
		}
	}
	return rooms
}

func Run() {
	fmt.Println("Day 23: Amphipod")

	inputString := input.ReadDay("day23")
	rooms := parseInput(inputString)
	_ = rooms

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
