package day07

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"math"
	"strconv"
	"strings"
)

func parseInput(str string) []int {
	pos := []int{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		for part := range strings.SplitSeq(line, ",") {
			if part == "" {
				continue
			}
			num, err := strconv.Atoi(part)
			if err != nil {
				log.Fatal(err)
			}
			pos = append(pos, num)
		}
	}
	return pos
}

func cheapestOutcome(crabs []int, sumNatural bool) int {
	maxPos := 0
	minPos := math.MaxInt
	outcome := math.MaxInt
	for _, pos := range crabs {
		if minPos > pos {
			minPos = pos
		}
		if maxPos < pos {
			maxPos = pos
		}
	}

	for t := minPos; t <= maxPos; t++ {
		cost := 0
		for _, pos := range crabs {
			var fuel int
			if t > pos {
				fuel = t - pos
			} else {
				fuel = pos - t
			}

			if sumNatural && fuel > 1 {
				// sum of n natural numbers: n*(n+1)/2
				fuel = fuel * (fuel + 1) / 2
			}

			cost += fuel
		}
		if cost < outcome {
			outcome = cost
		}
	}

	return outcome
}

func Run() {
	fmt.Println("Day 7: The Treachery of Whales")

	inputString := input.ReadDay("day07")
	crabs := parseInput(inputString)

	fmt.Printf("Part One: %d\n", cheapestOutcome(crabs, false))
	fmt.Printf("Part Two: %d\n", cheapestOutcome(crabs, true))
}
