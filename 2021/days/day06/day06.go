package day06

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func parseInput(input string) []int {
	nums := []int{}
	for split := range strings.SplitSeq(input, ",") {
		split = strings.TrimSpace(split)
		num, err := strconv.Atoi(split)
		if err != nil {
			log.Fatalln(err)
		}
		nums = append(nums, num)
	}
	return nums
}

func lanternfishGrowth(fish []int, days int) int {
	init := 8
	interval := 6
	for range days {
		temp := []int{}
		for i := range fish {
			if fish[i] == 0 {
				temp = append(temp, init)
				fish[i] = interval
			} else {
				fish[i]--
			}
		}
		fish = append(fish, temp...)
	}
	return len(fish)
}

func Run() {
	fmt.Println("Day 6: Lanternfish")

	inputString := input.ReadDay("day06")
	fish := parseInput(inputString)

	fmt.Printf("Part One: %d\n", lanternfishGrowth(fish, 80))
	fmt.Printf("Part Two: %d\n", lanternfishGrowth(fish, 256))
}
