package day09

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func parseInput(str string) [][]int {
	heightMap := [][]int{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		curr := []int{}
		for _, r := range line {
			num, err := strconv.Atoi(string(r))
			if err != nil {
				log.Fatal(err)
			}
			curr = append(curr, num)
		}
		heightMap = append(heightMap, curr)
	}
	return heightMap
}

func sumRiskLevelLowPoints(heightMap [][]int) int {
	sum := 0
	return sum
}

func Run() {
	fmt.Println("Day 9: Smoke Basin")

	inputString := input.ReadDay("day09")
	_ = parseInput(inputString)

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
