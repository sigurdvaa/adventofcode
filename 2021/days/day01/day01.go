package day01

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func parseInput(inputString string) []int {
	var depths []int
	for _, line := range strings.Split(inputString, "\n") {
		if line == "" {
			continue
		}

		num, err := strconv.Atoi(line)
		if err != nil {
			log.Fatal(err)
		}
		depths = append(depths, num)
	}
	return depths
}

func depthMeasurementIncreases(depths []int, window int) int {
	count := 0
	prev := 0

	for i := range depths {
		num := sum(depths[i : i+window])

		if prev != 0 && num > prev {
			count += 1
		}

		prev = num
	}

	return count
}

func sum(arr []int) int {
	_sum := 0
	for _, val := range arr {
		_sum += val
	}
	return _sum
}

func Run() {
	fmt.Println("Day 1: Sonar Sweep")

	inputString := input.ReadDay(1)
	depths := parseInput(inputString)

	fmt.Printf("Part One: %d\n", depthMeasurementIncreases(depths, 1))
	fmt.Printf("Part Two: %d\n", depthMeasurementIncreases(depths, 3))
}
