package day05

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Pos struct {
	x int
	y int
}

type Vent struct {
	start Pos
	end   Pos
}

func parsePos(str string) Pos {
	split := strings.Split(str, ",")

	x, err := strconv.Atoi(split[0])
	if err != nil {
		log.Fatalln(err)
	}

	y, err := strconv.Atoi(split[1])
	if err != nil {
		log.Fatalln(err)
	}

	return Pos{x, y}
}

func parseInput(stringInput string) []Vent {
	var vents []Vent
	for _, line := range strings.Split(stringInput, "\n") {
		if line == "" {
			continue
		}
		split := strings.Split(line, " -> ")
		start := parsePos(split[0])
		end := parsePos(split[1])
		vents = append(vents, Vent{start, end})
	}
	return vents
}

func Run() {
	fmt.Println("Day 5: Hydrothermal Venture")

	inputString := input.ReadDay("day05")
	vents := parseInput(inputString)
	_ = vents

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
