package day20

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

func parsePixel(p rune) bool {
	switch p {
	case '#':
		return true
	case '.':
		return false
	}
	panic(fmt.Sprintf("unknown pixel: %c", p))
}

func parseInput(str string) ([512]bool, [][]bool) {
	algo_parsed := false
	algo := [512]bool{}
	img := [][]bool{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}

		if algo_parsed {
		} else {
			for i, r := range line {
				algo[i] = parsePixel(r)
			}
			algo_parsed = true
		}

	}
	return algo, img
}

func Run() {
	fmt.Println("Day 20: Trench Map")

	inputString := input.ReadDay("day20")
	algo, img := parseInput(inputString)
	_, _ = algo, img

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
