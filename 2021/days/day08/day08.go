package day08

import (
	"aoc_2021/input"
	"fmt"
	"slices"
	"strings"
)

type display struct {
	signals []string
	output  []string
}

func parseInput(str string) []display {
	displays := []display{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		split := strings.Split(line, " | ")
		signals := strings.Split(split[0], " ")
		output := strings.Split(split[1], " ")
		displays = append(displays, display{signals, output})
	}
	return displays
}

func countEasyDigits(displays []display) int {
	uniqueLen := []int{2, 3, 4, 7}
	count := 0

	for _, disp := range displays {
		for _, digit := range disp.output {
			if slices.Contains(uniqueLen, len(digit)) {
				count += 1
			}
		}
	}

	return count
}

func Run() {
	fmt.Println("Day 8: Seven Segment Search")

	inputString := input.ReadDay("day08")
	displays := parseInput(inputString)

	fmt.Printf("Part One: %d\n", countEasyDigits(displays))
	fmt.Printf("Part Two: TODO\n")
}
