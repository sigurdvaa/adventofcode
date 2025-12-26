package day22

import (
	"aoc_2021/input"
	"fmt"
	"strconv"
	"strings"
)

type Cuboid struct {
	value  bool
	xStart int
	xEnd   int
	yStart int
	yEnd   int
	zStart int
	zEnd   int
}

func parseRange(str string) (start int, end int) {
	split := strings.Split(str[2:], "..")
	start, err := strconv.Atoi(split[0])
	if err != nil {
		panic(err)
	}
	end, err = strconv.Atoi(split[1])
	if err != nil {
		panic(err)
	}
	return start, end
}

func parseInput(str string) []Cuboid {
	procedure := []Cuboid{}

	for line := range strings.SplitSeq(str, "\n") {
		valueSplit := strings.Split(line, " ")
		value := false
		if valueSplit[0] == "on" {
			value = true
		}
		coordSplit := strings.Split(valueSplit[1], ",")
		xStart, xEnd := parseRange(coordSplit[0])
		yStart, yEnd := parseRange(coordSplit[1])
		zStart, zEnd := parseRange(coordSplit[2])
		procedure = append(procedure, Cuboid{value, xStart, xEnd, yStart, yEnd, zStart, zEnd})
	}

	return procedure
}

func runProcedure(procedure []Cuboid) int {
	return 0
}

func Run() {
	fmt.Println("Day 22: Reactor Reboot")

	inputString := input.ReadDay("day22")
	procedure := parseInput(inputString)

	fmt.Printf("Part One: %d\n", runProcedure(procedure))
	fmt.Printf("Part Two: TODO\n")
}
