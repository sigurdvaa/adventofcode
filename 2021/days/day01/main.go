package day01

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func Run() {
	fmt.Println("Day 1: Sonar Sweep")

	inputString := input.ReadDay(1)
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

	part1(depths)
	part2(depths)
}

func part1(depths []int) {
	count := 0

	for i := 1; i < len(depths); i++ {
		if depths[i] > depths[i-1] {
			count += 1
		}
	}

	fmt.Printf("Part One: %d\n", count)
}

func sum(arr []int) int {
	_sum := 0
	for _, val := range arr {
		_sum += val
	}
	return _sum
}

func part2(depths []int) {
	count := 0
	prev := 0

	for i := range depths {
		num := sum(depths[i : i+3])

		if prev != 0 && num > prev {
			count += 1
		}

		prev = num
	}

	fmt.Printf("Part Two: %d\n", count)
}
