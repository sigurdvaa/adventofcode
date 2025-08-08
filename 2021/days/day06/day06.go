package day06

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

const FISH_NEW = 8
const FISH_RESET = 6

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
	school := [FISH_NEW + 1]int{}
	for _, v := range fish {
		school[v] += 1
	}

	for range days {
		temp := [FISH_NEW + 1]int{}
		for i, v := range school {
			if i == 0 {
				temp[FISH_NEW] = v
				temp[FISH_RESET] = v
			} else {
				temp[i-1] += v
			}
		}
		school = temp
	}

	sum := 0
	for _, c := range school {
		sum += c
	}
	return sum
}

func Run() {
	fmt.Println("Day 6: Lanternfish")

	inputString := input.ReadDay("day06")
	fish := parseInput(inputString)

	fmt.Printf("Part One: %d\n", lanternfishGrowth(fish, 80))
	fmt.Printf("Part Two: %d\n", lanternfishGrowth(fish, 256))
}
