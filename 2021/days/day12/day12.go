package day12

import (
	"aoc_2021/input"
	"fmt"
	"slices"
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

func numPaths(caveMap CaveMap) int {
	done := [][]string{}
	queue := [][]string{{"start"}}
	visited := map[string]bool{}

	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]

		path := strings.Join(curr, "-")
		if visited[path] {
			continue
		}
		visited[path] = true

		if curr[len(curr)-1] == "end" {
			done = append(done, curr)
			continue
		}

		for _, nextCave := range caveMap[curr[len(curr)-1]] {
			// if small cave already in path, skip
			if nextCave[0] > 96 {
				if slices.Contains(curr, nextCave) {
					continue
				}
			}
			nextPath := append(append([]string{}, curr...), nextCave)
			queue = append(queue, nextPath)
		}
	}
	return len(done)
}

func Run() {
	fmt.Println("Day 12: Passage Pathing")

	inputString := input.ReadDay("day12")
	caveMap := parseInput(inputString)

	fmt.Printf("Part One: %d\n", numPaths(caveMap))
	fmt.Printf("Part Two: TODO\n")
}
