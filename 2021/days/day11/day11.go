package day11

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func parseInput(str string) [][]int {
	grid := [][]int{}

	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		row := []int{}
		for _, r := range line {
			num, err := strconv.Atoi(string(r))
			if err != nil {
				log.Fatal(err)
			}
			row = append(row, num)
		}
		grid = append(grid, row)
	}

	return grid
}

func bumpOctopus(grid [][]int, x int, y int) int {
	flashes := 0

	if grid[y][x] != -1 {
		grid[y][x] += 1
		if grid[y][x] > 9 {
			grid[y][x] = -1
			flashes += 1

			if y != 0 {
				flashes += bumpOctopus(grid, x, y-1)
				if x != 0 {
					flashes += bumpOctopus(grid, x-1, y-1)
				}
				if x != len(grid[y])-1 {
					flashes += bumpOctopus(grid, x+1, y-1)
				}
			}

			if x != 0 {
				flashes += bumpOctopus(grid, x-1, y)
			}
			if x != len(grid[y])-1 {
				flashes += bumpOctopus(grid, x+1, y)
			}

			if y != len(grid)-1 {
				flashes += bumpOctopus(grid, x, y+1)
				if x != 0 {
					flashes += bumpOctopus(grid, x-1, y+1)
				}
				if x != len(grid[y])-1 {
					flashes += bumpOctopus(grid, x+1, y+1)
				}
			}
		}
	}

	return flashes
}

func gameOfOctopus(grid [][]int, steps int) int {
	flashes := 0

	for i := 0; i < steps; i++ {
		for y := range grid {
			for x := range grid[y] {
				flashes += bumpOctopus(grid, x, y)
			}
		}
		for y := range grid {
			for x := range grid[y] {
				if grid[y][x] == -1 {
					grid[y][x] = 0
				}
			}
		}
	}

	return flashes
}

func gameOfOctopusFirstSync(grid [][]int) int {
	total := len(grid) * len(grid[0])
	steps := 0
	for {
		steps += 1
		flashes := 0
		for y := range grid {
			for x := range grid[y] {
				flashes += bumpOctopus(grid, x, y)
			}
		}

		if flashes == total {
			return steps
		}

		for y := range grid {
			for x := range grid[y] {
				if grid[y][x] == -1 {
					grid[y][x] = 0
				}
			}
		}
	}
}

func Run() {
	fmt.Println("Day 11: Dumbo Octopus")

	inputString := input.ReadDay("day11")

	grid := parseInput(inputString)
	fmt.Printf("Part One: %d\n", gameOfOctopus(grid, 100))

	grid = parseInput(inputString)
	fmt.Printf("Part Two: %d\n", gameOfOctopusFirstSync(grid))
}
