package day12

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

type CaveMap map[string][]string

func parseInput(str string) CaveMap {
	var caveMap CaveMap = make(CaveMap)

	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		split := strings.Split(line, "-")

		_, ok := caveMap[split[0]]
		if !ok {
			caveMap[split[0]] = []string{}
		}
		_, ok = caveMap[split[1]]
		if !ok {
			caveMap[split[1]] = []string{}
		}

		caveMap[split[0]] = append(caveMap[split[0]], split[1])
		caveMap[split[1]] = append(caveMap[split[1]], split[0])
	}

	return caveMap
}

func Run() {
	fmt.Println("Day 12: Passage Pathing")

	inputString := input.ReadDay("day12")
	caveMap := parseInput(inputString)
	_ = caveMap

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
