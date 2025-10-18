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

func diffCommonElement(polymer string, rules map[string]string, steps int) int {

	for range steps {
		nextPolymer := strings.Builder{}

		for i := range len(polymer) - 1 {
			nextPolymer.WriteString(polymer[i : i+1])
			val, ok := rules[polymer[i:i+2]]
			if ok {
				nextPolymer.WriteString(val)
			}
		}
		nextPolymer.WriteString(polymer[len(polymer)-1:])

		polymer = nextPolymer.String()
	}

	count := map[rune]int{}
	for _, char := range polymer {
		count[char] += 1
	}

	most := 0
	least := math.MaxInt
	for _, val := range count {
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

	fmt.Printf("Part One: %d\n", diffCommonElement(template, rules, 10))
	fmt.Printf("Part Two: %d\n", diffCommonElement(template, rules, 40))
}
