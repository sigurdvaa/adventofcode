package day14

import (
	"aoc_2021/input"
	"fmt"
	"math"
	"strings"
)

func parseInput(str string) (string, map[string]string) {
	lines := strings.Split(str, "\n")
	template := lines[0]
	rules := map[string]string{}

	for i := 2; i < len(lines); i++ {
		line := lines[i]
		if line == "" {
			continue
		}
		split := strings.Split(line, " -> ")
		rules[split[0]] = split[1]
	}

	return template, rules
}

func diffMostLeastCommonElements(polymer string, rules map[string]string, steps int) int {
	pairs := map[string]int{}
	for i := 0; i < len(polymer)-1; i++ {
		pairs[polymer[i:i+2]] += 1
	}

	for range steps {
		newPairs := map[string]int{}
		for pair, count := range pairs {
			ins := rules[pair]
			// add double count, but we'll only count first polymer in pairs afterwards
			newPairs[pair[0:1]+ins] += count
			newPairs[ins+pair[1:2]] += count
		}
		pairs = newPairs
	}

	countMap := map[string]int{}
	for pair, count := range pairs {
		// only count first polymer in pair
		countMap[pair[0:1]] += count
	}
	// add one extra to last polymer in chain
	countMap[polymer[len(polymer)-1:]] += 1

	most := 0
	least := math.MaxInt
	for _, val := range countMap {
		if val > most {
			most = val
		}
		if val < least {
			least = val
		}
	}

	return most - least
}

func Run() {
	fmt.Println("Day 14: Extended Polymerization")

	inputString := input.ReadDay("day14")
	template, rules := parseInput(inputString)

	fmt.Printf("Part One: %d\n", diffMostLeastCommonElements(template, rules, 10))
	fmt.Printf("Part Two: %d\n", diffMostLeastCommonElements(template, rules, 40))
}
