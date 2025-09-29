package day10

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

func parseInput(str string) [][]rune {
	nav := [][]rune{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		nav = append(nav, []rune(line))
	}
	return nav
}

func Run() {
	fmt.Println("Day 10: Syntax Scoring")

	inputString := input.ReadDay("day10")
	_ = inputString

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
