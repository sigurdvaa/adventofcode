package day09

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"sort"
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

type Pos struct {
	x int
	y int
}

func findBasinSize(heightMap [][]int, x int, y int) int {
	size := 0
	queue := []Pos{{x, y}}
	visited := map[Pos]bool{}

	for len(queue) > 0 {
		pos := queue[0]
		queue = queue[1:]
		if visited[pos] {
			continue
		}
		visited[pos] = true
		size += 1

		if pos.y > 0 && heightMap[pos.y-1][pos.x] < 9 {
			queue = append(queue, Pos{pos.x, pos.y - 1})
		}

		if pos.y < len(heightMap)-1 && heightMap[pos.y+1][pos.x] < 9 {
			queue = append(queue, Pos{pos.x, pos.y + 1})
		}

		if pos.x > 0 && heightMap[pos.y][pos.x-1] < 9 {
			queue = append(queue, Pos{pos.x - 1, pos.y})
		}

		if pos.x < len(heightMap[pos.y])-1 && heightMap[pos.y][pos.x+1] < 9 {
			queue = append(queue, Pos{pos.x + 1, pos.y})
		}
	}

	return size
}

func productLargestBasins(heightMap [][]int) int {
	basins := []int{}

	for y := range heightMap {
		for x := range heightMap[y] {
			if lowPoint(heightMap, x, y) {
				size := findBasinSize(heightMap, x, y)
				basins = append(basins, size)
			}
		}
	}

	sort.Ints(basins)
	product := 1
	for _, val := range basins[len(basins)-3:] {
		product *= val
	}
	return product
}

func Run() {
	fmt.Println("Day 9: Smoke Basin")

	inputString := input.ReadDay("day09")
	heightMap := parseInput(inputString)

	fmt.Printf("Part One: %d\n", sumRiskLevelLowPoints(heightMap))
	fmt.Printf("Part Two: %d\n", productLargestBasins(heightMap))
}
