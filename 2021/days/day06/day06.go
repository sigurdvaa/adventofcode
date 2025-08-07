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
		num, err := strconv.Atoi(split)
		if err != nil {
			log.Fatalln(err)
		}
		nums = append(nums, num)
	}
	return nums
}

func Run() {
	fmt.Println("Day 6: Lanternfish")

	inputString := input.ReadDay("day06")
	fish := parseInput(inputString)
	_ = fish

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
