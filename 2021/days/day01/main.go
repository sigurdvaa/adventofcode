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
	part1()
	part2()
}

func part1() {
	inputString := input.ReadDay(1)

	var depths []int
	count := 0
	prev := 0

	for _, line := range strings.Split(inputString, "\n") {
		if line == "" {
			continue
		}

		num, err := strconv.Atoi(line)
		if err != nil {
			log.Fatal(err)
		}
		depths = append(depths, num)

		if prev != 0 {
			if num > prev {
				count += 1
			}
		}
		prev = num

	}

	fmt.Printf("Part One: %d\n", count)
}

func part2() {
	fmt.Println("Part Two: TODO")
}
