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

func lowPoint(heightMap [][]int, x int, y int) bool {
	point := heightMap[y][x]

	if y > 0 && heightMap[y-1][x] <= point {
		return false
	}

	if y < len(heightMap)-1 && heightMap[y+1][x] <= point {
		return false
	}

	if x > 0 && heightMap[y][x-1] <= point {
		return false
	}

	if x < len(heightMap[y])-1 && heightMap[y][x+1] <= point {
		return false
	}

	return true
}

func sumRiskLevelLowPoints(heightMap [][]int) int {
	sum := 0

	for y := range heightMap {
		for x := range heightMap[y] {
			if lowPoint(heightMap, x, y) {
				sum += heightMap[y][x] + 1
			}
		}
	}

	return sum
}

func Run() {
	fmt.Println("Day 9: Smoke Basin")

	inputString := input.ReadDay("day09")
	heightMap := parseInput(inputString)

	fmt.Printf("Part One: %d\n", sumRiskLevelLowPoints(heightMap))
	fmt.Printf("Part Two: TODO\n")
}
